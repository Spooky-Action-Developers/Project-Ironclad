extern crate clap;


use clap::{App, AppSettings, SubCommand, Arg};

fn main() {
    let app_matches = App::new("Project Ironclad Secret Store")
        .version("0.2.0")
        .author("Evan Conley <econmang@gmail.com>\nJacob Cromwell <cromwellj@sou.edu>")
        .about("Ironclad is a command line utility to help store and manage secret credentials through AWS.")
        .subcommand(SubCommand::with_name("list")
                    .about("List DynamoDB tables associated with a given region.")
                    .arg(Arg::with_name("region")
                         .short("r")
                         .long("region")
                         .help("Sets region to list tables from. If not set, uses default region setting.")
                         .takes_value(true)
                         .value_name("REGION")
                         ))
        .subcommand(SubCommand::with_name("put")
                    .arg(Arg::with_name("identifier")
                         .required(true)
                         .help("Identifier with which to store the credential in AWS.")
                    )
                    .about("Store a credential through AWS.")
                    .arg(Arg::with_name("secret")
                         .help("String to be stored at command line")
                         //.required_unless("fileName")
                         )
                    .arg(Arg::with_name("fileName")
                         .short("f")
                         .long("file")
                         .takes_value(true)
                         .value_name("FILE")
                         .required(false)
                         .help("Path to the file to be stored."))
                    .arg(Arg::with_name("table")
                         .short("t")
                         .long("table")
                         .takes_value(true)
                         .value_name("TABLE")
                         .required(false)
                         .help("Specify which table to store secret credential in.")
                         ))
        .subcommand(SubCommand::with_name("delete")
                    .about("Delete specified secret from DynamoDB Table.")
                    .arg(Arg::with_name("tableName")
                         .short("t")
                         .long("table")
                         .help("Specifies name of table to find credential.")
                         .required(false)
                         .takes_value(true)
                         .value_name("TABLE")
                         )
                    .arg(Arg::with_name("identifier")
                         .required(true)
                         .help("Name of secreet to be deleted from DynamoDB Table.")
                         ))
        .subcommand(SubCommand::with_name("setup")
                    .about("Setup new DynamoDB Table through AWS.")
                    .arg(Arg::with_name("name")
                         .short("n")
                         .long("name")
                         .help("indicates name of table to be setup with default values")
                         .required(false)
                         .takes_value(true)
                         .value_name("NAME")
                         ))
        .subcommand(SubCommand::with_name("view")
                    .about("View credentials in specified DynamoDB Table.")
                    .arg(Arg::with_name("table")
                         .short("t")
                         .long("table")
                         .help("Indicates a table to view other than default.")
                         .required(false)
                         .takes_value(true)
                         .value_name("TABLE")
                         ))
        .subcommand(SubCommand::with_name("getall")
                    .about("Retrieve all secret credentials from a DynamoDB Table.")
                    .arg(Arg::with_name("table")
                         .short("t")
                         .long("table")
                         .help("Indicates a table to retrieve from.")
                         .required(false)
                         .takes_value(true)
                         .value_name("TABLE")
                         ))
        .subcommand(SubCommand::with_name("get")
                    .about("Retrieve a secret credential from a DynamoDB Table.")
                    .arg(Arg::with_name("identifier")
                         .help("Name of credential to be retrieved.")
                         .required(true))
                    .arg(Arg::with_name("table")
                         .short("t")
                         .long("table")
                         .help("Indicates a table to retrieve from.")
                         .required(false)
                         .takes_value(true)
                         .value_name("TABLE")
                         ))
        .setting(AppSettings::SubcommandRequired)
        .get_matches();


    if let Some(x) = app_matches.subcommand_matches("list") {
        if x.is_present("region") {
            println!("I'd be attempting to list tables from specified region.");
        }
        else {
            println!("I'd be attempting to list tables from default region.");
        }
    }
    else if let Some(x) = app_matches.subcommand_matches("put") {
        {
            if x.is_present("fileName") && x.is_present("table") {
                if x.is_present("secret") {
                    eprintln!("ERROR: Too many arguments for storage.");
                }
                else {
                    println!("Attempting to store file: {:?} with identifier {:?} in table: {:?}",
                         x.value_of("fileName").unwrap(), x.value_of("identifier").unwrap(), x.value_of("table").unwrap());
                }
            }
            else if x.is_present("fileName") {
                if x.is_present("secret") {
                    eprintln!("ERROR: Too many arguments for storage.");
                }
                else {
                println!("Attempting to store file: {:?} with identifier {:?} in default table.",
                         x.value_of("fileName").unwrap(), x.value_of("identifier").unwrap());
                }

            }
            else if x.is_present("table") {
                println!("Attmepting to store {:?} with name {:?} in table: {:?}",
                         x.value_of("secret").unwrap(), x.value_of("identifier").unwrap(),  x.value_of("table").unwrap());
            }
            else {
                println!("Attempting to store {:?} with name {:?} in default table",
                         x.value_of("secret").unwrap(), x.value_of("identifier").unwrap());
            }
        }
    }
    else if let Some(x) = app_matches.subcommand_matches("delete") {
        if x.is_present("tableName") {
            println!("I'd be attempting to delete {:?} from table {:?}.", x.value_of("identifier").unwrap(), x.value_of("tableName").unwrap());
        }
        else {
            println!("I'd be attempting to delete {:?} from default table.", x.value_of("identifier").unwrap());
        }
    }
    else if let Some(x) = app_matches.subcommand_matches("setup") {
        if x.is_present("name") {
            println!("I'd be attempting to create table with name: {:?}", x.value_of("name").unwrap());
        }
        else {
            println!("I'd be attempting to create default table \"ironclad-store\"");
        }
    }
    else if let Some(x) = app_matches.subcommand_matches("view") {
        if x.is_present("table") {
            println!("I'd be attempting to list the secrets in the table specified: {:?}", x.value_of("table").unwrap());
        }
        else {
            println!("I'd be attemtping to list the secrets in the default table.");
        }
    }
    else if let Some(x) = app_matches.subcommand_matches("getall") {
        if x.is_present("table"){
            println!("I'd be attempting to retrieve all secrets from: {:?}", x.value_of("table").unwrap());
        }
        else {
            println!("I'd be attempting to retrieve all secrets from default table.");
        }
    }
    else if let Some(x) = app_matches.subcommand_matches("get") {
        if x.is_present("table"){
            println!("I'd be attempting to retrieve {:?} from: {:?}", x.value_of("identifier").unwrap(), x.value_of("table").unwrap());
        }
        else {
            println!("I'd be attempting to retrieve {:?} from default table.", x.value_of("identifier").unwrap());
        }
    }
}
