use rusoto_core::region::Region;
use rusoto_dynamodb::*;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
use rusoto_kms::KmsClient;
use rusoto_kms::*;
use serde_json::to_string_pretty;
use std::collections::HashMap;

/// Options for encryption
#[derive(Clone, Debug, Default)]
pub struct EncryptOptions {
    /// The AWS region to use when calling KMS
    pub region: String,

    /// KMS key ID, ARN, alias or alias ARN
    pub key: String,

    /// AWS KMS encryption context
    pub encryption_context: HashMap<String, String>,
}

fn encrypt_secret(
    data: &String,
    options: &EncryptOptions,
) -> Result<EncryptResponse, EncryptError> {
    /*
     * Encrypts a secret. Returns, either, an EncryptResponse or EncryptError
     * representing whether or not the system was able to encrypt a secret.
     * This is a helper function called by put_item.
     *
     */

    let kms_client = KmsClient::simple(Region::default());
    let mut enc_req = EncryptRequest::default();
    enc_req.encryption_context = Some(options.encryption_context.clone());
    enc_req.key_id = options.key.clone();
    enc_req.plaintext = data.as_bytes().to_vec();

    let enc_res = kms_client.encrypt(&enc_req).sync().unwrap();
    Ok(enc_res)
}

pub fn put_item(table_name: &str, secret_name: &str, secret: &str, version_number: &str) -> () {
    /*
     * Puts a secret/item in the specified table. Does not return any explicit type, but calls
     * encrypt_secret to encrypt the secret using a KMS client. Then, places it in the DynamDB
     * table with the specified name and version number associated with it.
     * */

    match version_number.parse::<i32>() {
        Ok(version_num) => {
            let client = DynamoDbClient::simple(Region::default());
            let mut put_item_creator = PutItemInput::default();
            let mut map = HashMap::new();
            let attribute_name = "name".to_string();
            let attribute_secret = "secret".to_string();
            let attribute_number = "version".to_string();
            map.insert(attribute_name, val!(S => &secret_name));
            map.insert(attribute_number, val!(N =>  &version_num));

            let mut encryption_context = HashMap::new();
            encryption_context.insert("entity".to_owned(), "admin".to_owned());
            let options = EncryptOptions {
                encryption_context: encryption_context,
                key: "alias/ironclad".into(),
                region: "us-east-1".into(),
            };
            let data = secret.into();
            let ensec = encrypt_secret(&data, &options)
                .unwrap()
                .ciphertext_blob
                .unwrap();
            map.insert(attribute_secret, val!(B => ensec));
            put_item_creator.table_name = table_name.to_string();
            put_item_creator.item = map;
            client
                .put_item(&put_item_creator)
                .sync()
                .expect("Item push not working");
        }
        Err(_e) => {
            eprintln!("Incorrectly specified version number.");
        }
    }
}

pub fn get_item(table_name: &str, secret_name: &str, version_number: &str) -> () {
    /*
     * Function to retrieve secret/item data and print it to stdout in JSON format.
     * Does not return any explicit type. It creates a decryption context utilizing the
     * associated ironclad aliased KMS key and, upon retrieving the item associated with
     * the speicifed secret name and version number, decrypts it using this context before
     * printing to stdout
     *
     * */

    let client = DynamoDbClient::simple(Region::default());
    let mut get_item_input = GetItemInput::default();
    let mut map = HashMap::new();
    let attribute_name = "name".to_string();
    let attribute_number = "version".to_string();
    map.insert(attribute_name, val!(S => &secret_name));
    map.insert(attribute_number, val!(N =>  &version_number));

    get_item_input.table_name = table_name.to_string();
    get_item_input.key = map;
    let retrieved_item = client
        .get_item(&get_item_input)
        .sync()
        .expect("Failed to get requested Item");

    match retrieved_item.item {
        Some(got_item) => {
            let got_name = got_item.get("name").unwrap().clone();
            let secret_name = &*got_name.s.unwrap();
            let got_version = got_item.get("version").unwrap().clone();
            let version = got_version.n.unwrap();

            let got_secret = got_item.get("secret").unwrap().clone();
            let encrypted_secret = got_secret.b.unwrap();

            let mut decrypter = DecryptRequest::default();
            let mut decryption_context = HashMap::new();
            decryption_context.insert("entity".to_owned(), "admin".to_owned());

            decrypter.ciphertext_blob = encrypted_secret;
            decrypter.encryption_context = Some(decryption_context);
            let kms_client = KmsClient::simple(Region::default());
            let secret_digits = kms_client
                .decrypt(&decrypter)
                .sync()
                .unwrap()
                .plaintext
                .unwrap();

            let secret = "\n\t".to_string()
                + &(String::from_utf8_lossy(&secret_digits).replace("\\n", "\n\t"))
                + &("\n\r".to_string());
            let json_object = json!({
                                "Credential ID":
                                    {
                                        "Name": secret_name,
                                        "Version": version
                                    },
                                "Secret":
                                    [
                                        secret
                                    ]
                                });

            println!(
                "{}",
                to_string_pretty(&json_object)
                    .unwrap()
                    .replace("\\n\\t", "\n\t")
                    .replace("\\n", "\n\t")
                    .replace("\\r", "\r    ")
            );
        }
        None => {
            println!("Secret does not exist.");
        }
    }
}

pub fn get_all(table_name: &str) -> () {
    /*
     * Function to return all secrets from a given table.
     * Returns no explicit type, but recurrently calls get_item
     * function for each secret in a table, and prints them to stdout
     * in JSON format.
     *
     * */

    let client = DynamoDbClient::simple(Region::default());
    let mut scan_table_input = ScanInput::default();
    scan_table_input.table_name = table_name.to_string();
    let scan_output = client.scan(&scan_table_input).sync().expect("Scan Failed");
    println!(
        "There are {:?} items in {:?}\n",
        scan_output.count.unwrap(),
        scan_table_input.table_name
    );
    match scan_output.items {
        Some(vector) => {
            for secrets in vector {
                let mut secret = secrets.get("name").unwrap().clone();
                let secret_name = &*secret.s.unwrap();

                let mut versions = secrets.get("version").unwrap().clone();
                let version = versions.n.unwrap();
                get_item(table_name, secret_name, &version);
            }
        }
        None => {}
    }
}

pub fn delete_item(table_name: &str, secret_name: &str, secret_number: &str) -> () {
    /*
     * Function to delete an item from a given table utilizing its name and version number.
     * Does not return any explicit type.
     *
     * */

    let client = DynamoDbClient::simple(Region::default());
    let mut delete_item_ = DeleteItemInput::default();
    let mut map_delete = HashMap::new();
    let attribute = "name".to_string();
    let attribute_number = "version".to_string();
    map_delete.insert(attribute, val!(S => &secret_name));
    map_delete.insert(attribute_number, val!(N =>  &secret_number));
    delete_item_.table_name = table_name.to_string();
    delete_item_.key = map_delete;
    client
        .delete_item(&delete_item_)
        .sync()
        .expect("Delete Item not working");
}

pub fn list_items(table_name: &str) -> () {
    /*
     * Function to list all secrets available to retrieved in a given table.
     * This function does not explicitly return a type, instead printing the
     * secret name and version number of every secret in a DynamoDB Table.
     *
     * */

    let client = DynamoDbClient::simple(Region::default());
    let mut scan_table_input = ScanInput::default();
    scan_table_input.table_name = table_name.to_string();
    let scan_output = client.scan(&scan_table_input).sync().expect("Scan Failed");
    println!(
        "There are {:?} items in {:?}\n",
        scan_output.count.unwrap(),
        scan_table_input.table_name
    );
    match scan_output.items {
        Some(vector) => {
            let mut count = 1;
            for secrets in vector {
                let mut secret = secrets.get("name").unwrap().clone();
                let secret_name = &*secret.s.unwrap();

                let mut versions = secrets.get("version").unwrap().clone();
                let version = versions.n.unwrap();
                println!(
                    "Secret {}:\nName: {:?}\nVersion: {:?}",
                    count, secret_name, version
                );
                count = count + 1;
            }
        }
        None => {}
    }
}
