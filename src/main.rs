extern crate iron_lib;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;
extern crate clap;

//use rusoto_core::region::Region;
//use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use clap::{Arg, App, SubCommand};
use iron_lib::tables;

fn main() {

    let app_run = App::new("Ironclad Secret Store")
        .version("0.2.0")
        .author("Evan Conley <econmang@gmail.com>\nJacob Cromwell <cromwellj@sou.edu>")
        .about("ironclad is a utility for storing secrets through AWS and DynamoDB tables")
        .subcommand(SubCommand::with_name("put")
                    .about("Stores secret credentials into the AWS system")
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .help("Indicates file to be used for storage")
                         )
                    .arg(Arg::with_name("secret")
                         .help("The secret to be stored in the AWS system")
                         .required(true)
                    ))
        .subcommand(SubCommand::with_name("list")
                    .about("Lists the DynamoDB tables associated with a region.")
                    .arg(Arg::with_name("region")
                         .short("r")
                         .long("region")
                         .help("Specifies the server region user wishes to list the tables from")
                         .takes_value(true)
                         .value_name("name")
                         .number_of_values(1)
                         ))
        .subcommand(SubCommand::with_name("get")
                    .about("Retrieves secrets from AWS")
                    .arg(Arg::with_name("secret")
                         .help("Specifies the identifier of the secret to be retrieved")
                         .required(true)
                         ))
        .get_matches();
    

    /* Adding logic to system and utilizing parsed through informatinon */
    match app_run.subcommand_name() {
        Some ("list")   => {tables::list_tables();},
        Some("put")     => {},
        Some("get")     => {},
        _               => {},
    }

}
