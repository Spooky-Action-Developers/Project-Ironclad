
# Project-Ironclad
|Travis-CI  | ![travis build](https://travis-ci.org/Spooky-Action-Developers/Project-Ironclad.svg?branch=master) |
|--|--|

## Installation

Installation of the Project Ironclad command line tool will, ultimately, be handled
by Docker through automatic installation of a Dockerfile. This will allow us to
make sure all dependencies are shipped with the software. Until we have this solution working,
though, the current solution is to utilize git and requires Rust-Nightly be installed locally
alongside the Rust package manager, Cargo.

```
git clone https://github.com/Spooky-Action-Developers/Project-Ironclad.git
cargo build
cargo run
```
### Linux Installation Dependencies

Project Ironclad is reliant upon the rusoto AWS SDK. In order to utilize all features necessary, OpenSSL or equivalent is required.
For Linux users, there exists libssl as an alternative.

For Debian and Ubuntu, the following command with ensure the system is able to be installed:

```
$ sudo apt-get install build-essential libssl-dev 
```

Upon installation of dependencies, the system can be installed and utilized as intended.

## Description

Project Ironclad is a command line utility to effectively create, store and retrieve secrets (or credentials) through Amazon Web Services (AWS). The program utilizes the Rust programming language. In particular, it uses the Rust-Nightly branch of the Rust Language project and is built on top of Rusoto, a AWS SDK that utilizes the AWS API.

##Setup

### Working With Multiple AWS Accounts (profiles)

If you are working with multiple AWS accounts, you can set multiple profiles in the ```~/.aws/credentials```
file. For example:

```
[default]
aws_access_key_id = KIDTOYEXAMPLEASDAFAD
aws_secret_access_key = TOYEXAMPLE12341231

[switch]
aws_access_key_id = KIDTOYEXAMPLEASDAFAD
aws_secret_access_key = TOYEXAMPLE12341231
```

Then, by setting the ```AWS_PROFILE``` environment variable (i.e. ```export AWS_PROFILE=switch```) you can point ironclad
at the appropriate account to use for validation.

### Changing Default Region

In order to decide the default region associated with Project Ironclad, you can set it via the command (i.e. ```export AWS_DEFAULT_REGION=region_name```).
Alternatively, most commands take the -r flag to utilize a specific region if desired. If the region is not set via one of the previous methods, Project Ironcald with utilize us-east-1.

### Setting up KMS

```ironclad``` will not currently set up your KMS master key. To create a KMS master key,

1. Go to AWS console
2. Got to the IAM console/tab
3. Click "Encryption Keys" on the left
4. Click "Create Key". For the keys alias, put "ironclad"
5. Decide what IAM pricipals you want to be able to manage the key
6. On the "Key Usage Permissions" screen, pick the IAM users/roles that will be using Project Ironclad
   (Note: These can be changed as needed)
7. KMS setup complete
8. Note: Repeat this process for each region you wish to utilize Project Ironclad in

The system shall utilize the KMS client associated with the default region.
Then, if the default region is changed through ```AWS_DEFAULT_REGION```, it must have the KMS key setup in that region for Ironclad to work as intended.

## Usage

```
Project Ironclad Secret Store 0.2.0
Evan Conley <econmang@gmail.com>
Jacob Cromwell <cromwellj@sou.edu>
Ironclad is a command line utility to help store and manage secret credentials through AWS.

USAGE:
    ironclad <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete          Delete specified secret from DynamoDB Table.
    delete-table    Delete specified table from DynamoDB.
    get             Retrieve a secret credential from a DynamoDB Table.
    getall          Retrieve all secret credentials from a DynamoDB Table.
    help            Prints this message or the help of the given subcommand(s)
    list            List DynamoDB tables associated with a given region.
    put             Store a credential through AWS.
    setup           Setup new DynamoDB Table through AWS.
    view            View credentials in specified DynamoDB Table.




SUBCOMMAND BREAKDOWN:

delete
Delete specified secret from DynamoDB Table.

USAGE:
    ironclad delete [OPTIONS] <identifier>

FLAGS:
    -h, --help       Prints help information

OPTIONS:
    -t, --table <TABLE>    Specifies name of table to find credential.

ARGS:
    <identifier>    Name of secreet to be deleted from DynamoDB Table.

------------------------------------------------------------------------

get
Retrieve a secret credential from a DynamoDB Table.

USAGE:
    ironclad get [OPTIONS] <identifier>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --table <TABLE>    Indicates a table to retrieve from.

ARGS:
    <identifier>    Name of credential to be retrieved.

------------------------------------------------------------------------

getall
Retrieve all secret credentials from a DynamoDB Table.

USAGE:
    ironclad getall [OPTIONS]

FLAGS:
    -h, --help       Prints help information

OPTIONS:
    -t, --table <TABLE>    Indicates a table to retrieve from.

------------------------------------------------------------------------

list
List DynamoDB tables associated with a given region.

USAGE:
    ironclad list [OPTIONS]

FLAGS:
    -h, --help       Prints help information

OPTIONS:
    -r, --region <REGION>    Sets region to list tables from. If not set, uses default region setting.

------------------------------------------------------------------------

put
Store a credential through AWS.

USAGE:
    ironclad put [OPTIONS] <identifier> [secret]

FLAGS:
    -h, --help       Prints help information

OPTIONS:
    -f, --file <FILE>      Path to the file to be stored.
    -t, --table <TABLE>    Specify which table to store secret credential in.
    -v, --version <VERSION> Specify the version associated with this secret.
			    If version already exists, it will be overwritten on DynamoDB.

ARGS:
    <identifier>    Identifier with which to store the credential in AWS.
    <secret>        String to be stored at command line

------------------------------------------------------------------------

setup
Setup new DynamoDB Table through AWS.

USAGE:
    ironclad setup [OPTIONS]

FLAGS:
    -h, --help       Prints help information

OPTIONS:
    -n, --name <NAME>        Indicates name of table to be setup with default values
    -r, --region <REGION>    Indicates region in which to create table.

------------------------------------------------------------------------

view
View credentials in specified DynamoDB Table.

USAGE:
    ironclad view [OPTIONS]

FLAGS:
    -h, --help       Prints help information

OPTIONS:
    -t, --table <TABLE>    Indicates a table to view other than default.

-------------------------------------------------------------------------
delete-table
Delete specified table from AWS.

USAGE:
    ironclad delete-table <tableName>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --region <REGION>   Region of table to be deleted.

ARGS:
    <tableName>    Table to be deleted.

```

## Documentation

Documentation of the project and its releases, will be using the standard of semantic versioning. For more information on this release archetype, see [here](https://semver.org/).

The changes made to the system in the form of major and minor releases can be found in the [Project Ironclad Changelog](CHANGELOG.md).

## License

Project Ironclad is distributed under the terms of the Mozilla Public License.
See [License](LICENSE) for details.
