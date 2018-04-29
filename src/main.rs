extern crate clap;
extern crate iron_lib;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_dynamodb;

use clap::{App, AppSettings, Arg, SubCommand};
use iron_lib::tables;
use rusoto_core::region::Region;

fn main() {
    /*Set of region that can be specified. This allows to check values for get_region() functions
     * as well as to validate expected inputs in error messages.*/
    let regions = vec![
        "apnortheast1",
        "apnortheast2",
        "apsouth1",
        "apsoutheast1",
        "apsoutheast2",
        "cacentral1",
        "eucentral1",
        "euwest1",
        "euwest2",
        "euwest3",
        "saeast1",
        "useast1",
        "useast2",
        "uswest1",
        "uswest2",
        "usgovwest1",
        "cnnorth1",
        "cnnorthwest1",
    ];

    /*Start of program logic. This will be used to gather and parse passed arguments from the user.
     * Assesses subcommand, flags, and values passed to each.
     *
     * The use of clap autogenerated help documentation for overall run and subcommands.*/
    let app_matches = App::new("Project Ironclad Secret Store")
        //Version and authoring information
        .version("0.2.0")
        .author("Evan Conley <econmang@gmail.com>\nJacob Cromwell <cromwellj@sou.edu>")
        .about("Ironclad is a command line utility to help store and manage secret credentials through AWS.")
        //Subcommand information/flags for `list` subcommand
        //lists tables in specified region server
        .subcommand(SubCommand::with_name("list")
                    .about("List DynamoDB tables associated with a given region.")
                    .arg(Arg::with_name("region")
                         .short("r")
                         .long("region")
                         .help("Sets region to list tables from. If not set, uses default region setting.")
                         .takes_value(true)
                         .value_name("REGION")
                         ))
        //Subcommand information/flags for `put` subcommand
        //stores credential into specified table
        .subcommand(SubCommand::with_name("put")
                    .arg(Arg::with_name("identifier")
                         .required(true)
                         .help("Identifier with which to store the credential in AWS.")
                    )
                    .arg(Arg::with_name("secret")
                         .help("String to be stored at command line")
                         //.required_unless("fileName")
                         )
                    .about("Store a credential through AWS.")
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
        //Subcommand information/flags for `delete` subcommand
        //deletes credential from table
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
        //Subcommand information/flags for `setup` subcommand
        //creates table
        .subcommand(SubCommand::with_name("setup")
                    .about("Setup new DynamoDB Table through AWS.")
                    .arg(Arg::with_name("name")
                         .short("n")
                         .long("name")
                         .help("Indicates name of table to be setup.")
                         .required(false)
                         .takes_value(true)
                         .value_name("NAME")
                         )
                    .arg(Arg::with_name("region")
                         .short("r")
                         .long("region")
                         .help("Indicates region in which to create table.")
                         .required(false)
                         .takes_value(true)
                         .value_name("REGION")
                         )
                    )
        //Subcommand information/flags for `view` subcommand
        //Lists the identifiers of secrets in table
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
        //Subcommand information/flags for `getall` subcommand
        //Retrieves all secrets from a table
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
        //Subcommand information/flags for `get` command
        //Retrieves specified secret from table
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
        //Subcommand information/flags for `delete-table` command
        //Deletes specified table from AWS
        .subcommand(SubCommand::with_name("delete-table")
                    .about("Delete specified table from AWS.")
                    .arg(Arg::with_name("tableName")
                         .help("Table to be deleted.")
                         .required(true))
                    .arg(Arg::with_name("region")
                         .short("r")
                         .long("region")
                         .help("Region of table to be deleted.")
                         .required(false)
                         .takes_value(true)
                         .value_name("REGION"))
                    )
        //Required subcommand
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    /*Program logic:
     * Uses subcommand_matches to locate which subcommand was utilized
     *      Error if none was used, or if typed incorrectly
     * After subcommand found, parses through flag/argument data
     *      Error if required arguments missing or flag used incorrectly
     * Returns specified information with helpful messages*/

    if let Some(x) = app_matches.subcommand_matches("list") {
        if x.is_present("region") {
            //check if region was set
            let reg = tables::get_region(x.value_of("region").unwrap());
            match reg {
                Some(reg) => {
                    tables::list_tables_region(reg);
                } //if region correctly parsed, list tables in region
                None => {
                    //else: display error informing what values can be used
                    eprintln!("Error: Region not correctly specified...\n");
                    let mut reg_list_string = "";
                    eprintln!("Must be in list:{}", reg_list_string);
                    for region in regions {
                        println!("{}", region);
                    }
                }
            }
        } else {
            tables::list_tables_default(); //region flag not set, list default region
        }
    } else if let Some(x) = app_matches.subcommand_matches("put") {
        {
            if x.is_present("fileName") && x.is_present("table") {
                if x.is_present("secret") {
                    eprintln!("ERROR: Too many arguments for storage.");
                } else {
                    println!(
                        "Attempting to store file: {:?} with identifier {:?} in table: {:?}",
                        x.value_of("fileName").unwrap(),
                        x.value_of("identifier").unwrap(),
                        x.value_of("table").unwrap()
                    );
                }
            } else if x.is_present("fileName") {
                if x.is_present("secret") {
                    eprintln!("ERROR: Too many arguments for storage.");
                } else {
                    println!(
                        "Attempting to store file: {:?} with identifier {:?} in default table.",
                        x.value_of("fileName").unwrap(),
                        x.value_of("identifier").unwrap()
                    );
                }
            } else if x.is_present("table") {
                println!(
                    "Attmepting to store {:?} with name {:?} in table: {:?}",
                    x.value_of("secret").unwrap(),
                    x.value_of("identifier").unwrap(),
                    x.value_of("table").unwrap()
                );
            } else {
                if x.is_present("secret") {
                    println!(
                        "Attempting to store {:?} with name {:?} in default table",
                        x.value_of("secret").unwrap(),
                        x.value_of("identifier").unwrap()
                    );
                } else {
                    eprintln!("ERROR: Missing required argument: 'secret' for storage.");
                }
            }
        }
    } else if let Some(x) = app_matches.subcommand_matches("delete") {
        if x.is_present("tableName") {
            println!(
                "I'd be attempting to delete {:?} from table {:?}.",
                x.value_of("identifier").unwrap(),
                x.value_of("tableName").unwrap()
            );
        } else {
            println!(
                "I'd be attempting to delete {:?} from default table.",
                x.value_of("identifier").unwrap()
            );
        }
    } else if let Some(x) = app_matches.subcommand_matches("setup") {
        if x.is_present("name") && x.is_present("region") {
            let reg = tables::get_region(x.value_of("region").unwrap());
            match reg {
                Some(reg) => {
                    tables::table_create_reg_name(reg, x.value_of("name").unwrap());
                } //if region correctly parsed, list tables in region
                None => {
                    //else: display error informing what values can be used
                    eprintln!("Error: Region not correctly specified...\n");
                    let mut reg_list_string = "";
                    eprintln!("Must be in list:{}", reg_list_string);
                    for region in regions {
                        println!("{}", region);
                    }
                }
            }
        } else if x.is_present("name") {
            tables::table_create_reg_name(Region::UsWest2, x.value_of("name").unwrap())
        } else if x.is_present("region") {
            let reg = tables::get_region(x.value_of("region").unwrap());
            match reg {
                Some(reg) => {
                    tables::table_create_reg_name(reg, "ironclad-store");
                } //if region correctly parsed, list tables in region
                None => {
                    //else: display error informing what values can be used
                    eprintln!("Error: Region not correctly specified...\n");
                    let mut reg_list_string = "";
                    eprintln!("Must be in list:{}", reg_list_string);
                    for region in regions {
                        println!("{}", region);
                    }
                }
            }
        } else {
            tables::table_create_default();
        }
    } else if let Some(x) = app_matches.subcommand_matches("view") {
        if x.is_present("table") {
            println!(
                "I'd be attempting to list the secrets in the table specified: {:?}",
                x.value_of("table").unwrap()
            );
        } else {
            println!("I'd be attemtping to list the secrets in the default table.");
        }
    } else if let Some(x) = app_matches.subcommand_matches("getall") {
        if x.is_present("table") {
            println!(
                "I'd be attempting to retrieve all secrets from: {:?}",
                x.value_of("table").unwrap()
            );
        } else {
            println!("I'd be attempting to retrieve all secrets from default table.");
        }
    } else if let Some(x) = app_matches.subcommand_matches("get") {
        if x.is_present("table") {
            println!(
                "I'd be attempting to retrieve {:?} from: {:?}",
                x.value_of("identifier").unwrap(),
                x.value_of("table").unwrap()
            );
        } else {
            println!(
                "I'd be attempting to retrieve {:?} from default table.",
                x.value_of("identifier").unwrap()
            );
        }
    } else if let Some(x) = app_matches.subcommand_matches("delete-table") {
        if x.is_present("region") {
            let reg = tables::get_region(x.value_of("region").unwrap());
            match reg {
                Some(reg) => {
                    let mut delete_table_name = x.value_of("tableName").unwrap();
                    tables::table_deleter_reg(reg,delete_table_name);
                } //if region correctly parsed, list tables in region
                None => {
                    //else: display error informing what values can be used
                    eprintln!("Error: Region not correctly specified...\n");
                    let mut reg_list_string = "";
                    eprintln!("Must be in list:{}", reg_list_string);
                    for region in regions {
                        println!("{}", region);
                    }
                }
            }
        }
        else {
            let mut delete_table_name = x.value_of("tableName").unwrap();
            tables::table_deleter(delete_table_name);
        }
    }
}
