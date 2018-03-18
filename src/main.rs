extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;

//use rusoto_credential::{ProfileProvider,ProvideAwsCredentials};
use rusoto_core::region::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

fn main() {
    let client = DynamoDbClient::simple(Region::UsWest2);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(&list_tables_input).sync() {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");

                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                },
                None => println!("No tables in database!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }

}
