extern crate iron_lib;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;
extern crate clap;

use rusoto_core::region::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use clap::{Arg, App};
use iron_lib::tables;
fn main() {
    let matches = App::new("Ironclad Secret Store")
        .version("0.1.0")
        .author("Evan Conley <econmang@gmail.com>\nJacob Cromwell <cromwellj@sou.edu>")
        .about("ironclad is a utility for storing secret through AWS and DynamoDB tables")
        .arg(Arg::with_name("action")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("action to perform in AWS System"))
        .get_matches();

    let secret = matches.value_of("action").unwrap();
    match secret {
        "list" => tables::list_tables(),
        _ => println!("The action you wish to perform is {}", secret),
    };
}
