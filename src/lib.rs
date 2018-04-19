extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;

use rusoto_core::region::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

    pub mod helpers {
        #[macro_export]
        macro_rules! attributes {
            ($($val:expr => $attr_type:expr),*) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push(AttributeDefinition { attribute_name: String::from($val), attribute_type: String::from($attr_type) });
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
                        temp_vec.push(KeySchemaElement { key_type: String::from($key_type), attribute_name: String::from($name) });
                    )*
                    temp_vec
                }
            }
        }

        #[macro_export]
        macro_rules! val {
        	(B => $val:expr) => (
        	    {
        	    	let mut attr = AttributeValue::default();
        	    	attr.b = Some($val);
        	    	attr
        	    }
        	);
        	(S => $val:expr) => (
        	    {
        			let mut attr = AttributeValue::default();
        			attr.s = Some($val.to_string());
        			attr
        		}
        	);
        	(N => $val:expr) => (
        	    {
        	    	let mut attr = AttributeValue::default();
        	    	attr.n = Some($val.to_string());
        	    	attr
        	    }
        	);
        }
    }

pub mod tables {
    use rusoto_core::region::Region;
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
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

    pub fn get_region(reg: &str) -> Option<Region> {
        match reg {
            "apnortheast1" =>  {return Some(Region::ApNortheast1)},
            "apnortheast2" =>  {return Some(Region::ApNortheast2)},
            "apsout1"      =>  {return Some(Region::ApSouth1)},
            "apsoutheast1" =>  {return Some(Region::ApSoutheast1)},
            "apsoutheast2" =>  {return Some(Region::ApSoutheast2)},
            "cacentral1"   =>  {return Some(Region::CaCentral1)},
            "eucentral1"   =>  {return Some(Region::EuCentral1)},
            "euwest1"      =>  {return Some(Region::EuWest1)},
            "euwest2"      =>  {return Some(Region::EuWest2)},
            "euwest3"      =>  {return Some(Region::EuWest3)},
            "saeast1"      =>  {return Some(Region::SaEast1)},
            "useast1"      =>  {return Some(Region::UsEast1)},
            "useast2"      =>  {return Some(Region::UsEast2)},
            "uswest1"      =>  {return Some(Region::UsWest1)},
            "uswest2"      =>  {return Some(Region::UsWest2)},
            "usgovwest1"   =>  {return Some(Region::UsGovWest1)},
            "cnnorth1"     =>  {return Some(Region::CnNorth1)},
            "cnnorthwest1" =>  {return Some(Region::CnNorthwest1)},
            _               =>  {None}
    }
}

    pub fn list_tables_region(region: Region) -> () {
    let client = DynamoDbClient::simple(region);
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
        }
    }
    }
}

pub mod CreateTableInputHelper{
    use std::str;
    use rusoto_core::ProvideAwsCredentials;
    use rusoto_core::region::Region;
    use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
    use rusoto_dynamodb::*;
    type PositiveLongObject = i64;

    macro_rules! try_opt {
    ($expr:expr) => (match $expr {
        ::std::option::Option::Some(ref val) => val,
        ::std::option::Option::None => return None
    })
}

pub struct DynamoDbHelper<P> where P: ProvideAwsCredentials {
    client: DynamoDbClient<P>,
}

impl <P: ProvideAwsCredentials> DynamoDbHelper<P> {
    pub fn new(credentials: P, region: Region) -> DynamoDbHelper<P> {
        DynamoDbHelper { client: DynamoDbClient::new(credentials, region) }
    }

    pub fn list_tables(&mut self) -> AwsResult<ListTablesOutput> {
        let mut req = ListTablesInput::default();
        self.client.list_tables(&req)
    }

    pub fn create_table(&mut self, input: &CreateTableInput) -> AwsResult<CreateTableOutput> {
        self.client.create_table(input)
    }

    pub fn describe_table(&mut self, name: &str) -> AwsResult<DescribeTableOutput> {
        let mut input = DescribeTableInput::default();
        input.table_name = String::from(name);
        self.client.describe_table(&input)
    }

    pub fn delete_table(&mut self, name: &str) -> AwsResult<DeleteTableOutput> {
        let mut input = DeleteTableInput::default();
        input.table_name = String::from(name);
        self.client.delete_table(&input)
    }

    pub fn put_item(&mut self, input: &PutItemInput) -> AwsResult<PutItemOutput> {
        self.client.put_item(input)
    }

    pub fn get_item(&mut self, input: &GetItemInput) -> AwsResult<GetItemOutput> {
        self.client.get_item(input)
    }
}

pub trait PutItemInputHelper {
    fn new() -> PutItemInput;
}

impl PutItemInputHelper for PutItemInput {
    fn new() -> PutItemInput {
        PutItemInput::default()
    }
}

pub trait CreateTableInputHelper {
    fn new() -> CreateTableInput;
    fn with_name(mut self, table_name: &str) -> CreateTableInput;
    fn with_write_capacity(mut self, write_capacity: PositiveLongObject ) -> CreateTableInput;
    fn with_read_capacity(mut self, read_capacity: PositiveLongObject) -> CreateTableInput;
    fn with_attributes(mut self, attributes: Vec<AttributeDefinition>) -> CreateTableInput;
    fn with_key_schema(mut self, key_schema: Vec<KeySchemaElement>) -> CreateTableInput;
    fn add_attribute<N: Into<String>, T: Into<String>>(mut self,
                                                       name: N,
                                                       attr_type: T)
                                                       -> CreateTableInput;
}

impl CreateTableInputHelper for CreateTableInput {
    fn new() -> CreateTableInput {
        CreateTableInput::default()
    }

    fn with_name(mut self, table_name: &str) -> CreateTableInput {
        self.table_name = String::from(table_name);
        self
    }

    fn with_write_capacity(mut self, write_capacity: PositiveLongObject) -> CreateTableInput {
        self.provisioned_throughput.write_capacity_units = write_capacity;
        self
    }

    fn with_read_capacity(mut self, read_capacity: PositiveLongObject) -> CreateTableInput {
        self.provisioned_throughput.read_capacity_units = read_capacity;
        self
    }

    fn with_attributes(mut self, attributes: Vec<AttributeDefinition>) -> CreateTableInput {
        self.attribute_definitions = attributes;
        self
    }

    fn with_key_schema(mut self, key_schema: Vec<KeySchemaElement>) -> CreateTableInput {
        self.key_schema = key_schema;
        self
    }

    fn add_attribute<N: Into<String>, T: Into<String>>(mut self,
                                                       name: N,
                                                       attr_type: T)
                                                       -> CreateTableInput {
        self.attribute_definitions.push(AttributeDefinition {
            attribute_name: name.into(),
            attribute_type: attr_type.into(),
        });
        self
    }
}

pub trait DescribeTableOutputHelper {
    fn get_status(&self) -> Option<String>;
}

impl DescribeTableOutputHelper for DescribeTableOutput {
    fn get_status(&self) -> Option<String> {
        let table = try_opt!(self.table);
        Some(try_opt!(table.table_status).to_string())
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
        println!("Tables in database:\ncredential-store");
    }
}
