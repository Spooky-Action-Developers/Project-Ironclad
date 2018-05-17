extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_dynamodb;
extern crate rusoto_kms;

pub mod tables {
    use rusoto_core::region::Region;
    use rusoto_dynamodb::*;
    use rusoto_dynamodb::{CreateTableInput, DynamoDb, DynamoDbClient, ListTablesInput, ScanInput};
    use rusoto_kms::KmsClient;
    use rusoto_kms::*;
    use std::collections::HashMap;

    #[macro_export]
    macro_rules! attributes {
        ($($val:expr => $attr_type:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push(AttributeDefinition { attribute_name: String::from($val),
                                    attribute_type: String::from($attr_type) });
                )*
                temp_vec
            }
        }
    }

    #[macro_export]
    macro_rules! key_schema {
        ($($name:expr => $key_type:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push(KeySchemaElement { key_type: String::from($key_type),
                                    attribute_name: String::from($name) });
                )*
                temp_vec
            }
        }
    }
    #[macro_export]
    macro_rules! val {
        (B => $val:expr) => {{
            let mut attr = AttributeValue::default();
            attr.b = Some($val);
            attr
        }};
        (S => $val:expr) => {{
            let mut attr = AttributeValue::default();
            attr.s = Some($val.to_string());
            attr
        }};
        (N => $val:expr) => {{
            let mut attr = AttributeValue::default();
            attr.n = Some($val.to_string());
            attr
        }};
    }

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
        let kms_client = KmsClient::simple(Region::UsWest2);
        let mut enc_req = EncryptRequest::default();
        enc_req.encryption_context = Some(options.encryption_context.clone());
        enc_req.key_id = options.key.clone();
        enc_req.plaintext = data.as_bytes().to_vec();

        let enc_res = kms_client.encrypt(&enc_req).sync().unwrap();
        Ok(enc_res)
    }

    pub fn list_tables_default() -> () {
        // First grabbing user credentials from .aws/credentials file
        let client = DynamoDbClient::simple(Region::UsWest2);
        let list_tables_input: ListTablesInput = Default::default();

        match client.list_tables(&list_tables_input).sync() {
            Ok(output) => match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");

                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                }
                None => println!("No tables in database!"),
            },
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }

    pub fn get_region(reg: &str) -> Option<Region> {
        match reg {
            "ap-northeast-1" => return Some(Region::ApNortheast1),
            "ap-northeast-2" => return Some(Region::ApNortheast2),
            "ap-south-1" => return Some(Region::ApSouth1),
            "ap-southeast-1" => return Some(Region::ApSoutheast1),
            "ap-southeast-2" => return Some(Region::ApSoutheast2),
            "ca-central-1" => return Some(Region::CaCentral1),
            "eu-central-1" => return Some(Region::EuCentral1),
            "eu-west-1" => return Some(Region::EuWest1),
            "eu-west-2" => return Some(Region::EuWest2),
            "eu-west-3" => return Some(Region::EuWest3),
            "sa-east-1" => return Some(Region::SaEast1),
            "us-east-1" => return Some(Region::UsEast1),
            "us-east-2" => return Some(Region::UsEast2),
            "us-west-1" => return Some(Region::UsWest1),
            "us-west-2" => return Some(Region::UsWest2),
            "us-govwest-1" => return Some(Region::UsGovWest1),
            "cn-north-1" => return Some(Region::CnNorth1),
            "cn-northwest-1" => return Some(Region::CnNorthwest1),
            _ => None,
        }
    }

    pub fn list_tables(region: Region) -> () {
        let client = DynamoDbClient::simple(region);
        let list_tables_input: ListTablesInput = Default::default();

        match client.list_tables(&list_tables_input).sync() {
            Ok(output) => match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");

                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                }
                None => println!("No tables in database!"),
            },
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }

    pub fn table_creator(reg: Region, name: &str) -> () {
        let client = DynamoDbClient::simple(reg);
        let tname = name.to_string();
        let mut table_creator = CreateTableInput::default();
        println!("Creating table {}", tname);
        let read_capacity = 1;
        let write_capacity = 1;
        table_creator.table_name = tname;
        table_creator.provisioned_throughput.read_capacity_units = read_capacity;
        table_creator.provisioned_throughput.write_capacity_units = write_capacity;
        table_creator.key_schema = key_schema!("name" => "HASH","version" => "RANGE");
        table_creator.attribute_definitions = attributes!("name" => "S","version" => "N");
        client
            .create_table(&table_creator)
            .sync()
            .expect("Failed to create table.");
        println!("Table name is {}", table_creator.table_name);
    }

    pub fn table_deleter(reg: Region, name: &str) -> () {
        let client = DynamoDbClient::simple(reg);
        let mut table_deleter = DeleteTableInput::default();
        let new_delete_table = name.to_string();
        table_deleter.table_name = new_delete_table;
        client
            .delete_table(&table_deleter)
            .sync()
            .expect("Delete Table Failed");
        println!("Successfully deleted: {:?}", table_deleter.table_name);
    }

    pub fn delete_item(table_name: &str, secret_name: &str, secret_number: &str) -> () {
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

    pub fn put_item(table_name: &str, secret_name: &str, secret: &str, version_number: &str) -> () {
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
                    key: "alias/TestKey".into(),
                    region: "us-west-2".into(),
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

    pub fn list_items(table_name: &str) -> () {
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
}
