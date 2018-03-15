use std::default::Default;
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_dynamodb::{DynamoDbClient, ListTablesInput};

pub fn auth_user() -> () {
    let P = DefaultCredentialsProvider::new().unwrap();
}
