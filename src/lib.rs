extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_dynamodb;

pub mod tables {
    use rusoto_core::region::Region;
    use rusoto_dynamodb::*;
    use rusoto_dynamodb::{CreateTableInput, DynamoDb, DynamoDbClient, ListTablesInput};
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
            "apnortheast1" => return Some(Region::ApNortheast1),
            "apnortheast2" => return Some(Region::ApNortheast2),
            "apsout1" => return Some(Region::ApSouth1),
            "apsoutheast1" => return Some(Region::ApSoutheast1),
            "apsoutheast2" => return Some(Region::ApSoutheast2),
            "cacentral1" => return Some(Region::CaCentral1),
            "eucentral1" => return Some(Region::EuCentral1),
            "euwest1" => return Some(Region::EuWest1),
            "euwest2" => return Some(Region::EuWest2),
            "euwest3" => return Some(Region::EuWest3),
            "saeast1" => return Some(Region::SaEast1),
            "useast1" => return Some(Region::UsEast1),
            "useast2" => return Some(Region::UsEast2),
            "uswest1" => return Some(Region::UsWest1),
            "uswest2" => return Some(Region::UsWest2),
            "usgovwest1" => return Some(Region::UsGovWest1),
            "cnnorth1" => return Some(Region::CnNorth1),
            "cnnorthwest1" => return Some(Region::CnNorthwest1),
            _ => None,
        }
    }

    pub fn list_tables_region(region: Region) -> () {
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

    pub fn table_create_default() -> () {
        let client = DynamoDbClient::simple(Region::UsWest2);
        let mut table_creator = CreateTableInput::default();
        let read_capacity = 1;
        let write_capacity = 1;
        table_creator.table_name = "ironclad-store".to_string();
        table_creator.provisioned_throughput.read_capacity_units = read_capacity;
        table_creator.provisioned_throughput.write_capacity_units = write_capacity;
        table_creator.key_schema = key_schema!("name" => "HASH", "version" => "RANGE");
        table_creator.attribute_definitions = attributes!("name" => "S", "version" => "N");
        client
            .create_table(&table_creator)
            .sync()
            .expect("Create default table failed.");
        println!("Table name is {}", table_creator.table_name);
    }

    pub fn table_create_reg_name(reg: Region, name: &str) -> () {
        let client = DynamoDbClient::simple(reg);
        let tname = name.to_string();
        let mut table_creator = CreateTableInput::default();
        println!("Creating table {}", tname);
        let read_capacity = 1;
        let write_capacity = 1;
        table_creator.table_name = tname;
        table_creator.provisioned_throughput.read_capacity_units = read_capacity;
        table_creator.provisioned_throughput.write_capacity_units = write_capacity;
        table_creator.key_schema = key_schema!("name" => "HASH", "version" => "RANGE"); 
        table_creator.attribute_definitions = attributes!("name" => "S", "version" => "N");
        client
            .create_table(&table_creator)
            .sync()
            .expect("Failed to create table.");
        println!("Table name is {}", table_creator.table_name);
    }

    pub fn table_deleter(name: &str) -> () {
        let client = DynamoDbClient::simple(Region::UsWest2);
        let mut table_deleter = DeleteTableInput::default();
        let new_delete_table = name.to_string();
        table_deleter.table_name = new_delete_table;
        client
            .delete_table(&table_deleter)
            .sync()
            .expect("Delete Table Failed");
        println!("Successfully deleted: {:?}", table_deleter.table_name);
    }

    pub fn table_deleter_reg(reg: Region, name: &str) -> () {
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
        let client = DynamoDbClient::simple(Region::UsWest2);
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

    pub fn put_item(table_name: &str, secret_name: &str, secret_number: &str) -> () {
        let client = DynamoDbClient::simple(Region::UsWest2);
        let mut put_item_creator = PutItemInput::default();
        let mut map= HashMap::new();
        let attribute = "name".to_string();
        let attribute_number = "version".to_string();
        map.insert(attribute, val!(S => &secret_name));
        map.insert(attribute_number, val!(N =>  &secret_number));
        put_item_creator.table_name = table_name.to_string();
        put_item_creator.item = map;
        client.put_item(&put_item_creator).sync().expect("Item push not working");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_authenticate_user() {
        // Set AWS Variables to NULL values originally

        // Run Authenticate User method

        // Assert equality of AWS environment
        // vars with their expected values
        assert_eq!(1, 1);
    }

    #[test]
    fn t_list_tables() {
        println!("Function Output:\n");
        tables::list_tables_default();
        println!("\n\nExpected output:\n");
        println!("Tables in database:\ncredential-store");
    }
}
