use rusoto_core::region::Region;
use rusoto_dynamodb::*;
use rusoto_dynamodb::{CreateTableInput, DynamoDb, DynamoDbClient, ListTablesInput};

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

pub fn get_region(reg: &str) -> Option<Region> {
    match reg {
        "default" => return Some(Region::default()),
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
