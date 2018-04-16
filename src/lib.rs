extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;

pub mod tables {
    use rusoto_core::region::Region;
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput,DescribeTableInput};

    pub fn list_tables_default() -> () {
        // First grabbing user credentials from .aws/credentials file
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

    pub fn list_table_region() -> () {
    //First grabbing user credentials from .aws/credentials file
    let client = DynamoDbClient::simple(Region::UsWest2);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(&list_tables_input).sync() {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in databse:");

                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                },
                None => println!("No tables in database!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_authenticate_user(){
        // Set AWS Variables to NULL values originally

        // Run Authenticate User method

        // Assert equality of AWS environment
        // vars with their expected values
        assert_eq!(1,1);
    }

    #[test]
    fn t_list_tables() {
        println!("Function Output:\n");
        tables::list_tables_default();
        println!("\n\nExpected output:\n");
        println!("Tables in databse:\ncredential-store");
    }
}
