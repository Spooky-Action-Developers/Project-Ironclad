# Project Ironclad Changelog

All notable changes to the project will be documented here.

## 0.4.1 - 2018-05-17
____________________________________________________________________________

## Added
  - New ```put``` functionality to store with a secret value
  - Added KMS to encrypt and decrypt on ```put``` and ```get```
  - Ability to ```get``` a secret
  - Ability to store a secret from a file
## Changed
  - Default region settings for all functions in lib.rs
  - Output messages to storage and delete in main.rs
  - Travis file to accomodate changes to rustfmt
  - Main functionality for put subcommand takes secret value
  - Updated put_item logic to store secret value field in DynamDB.
## Removed
  - Redundant codebase for default vs. regional values for functions in lib.rs
  - Refactored main code to remove redundant conditionals, increasing efficiency of checks

## 0.3.1 - 2018-05-03
____________________________________________________________________________

### Added
  - 
### Changed
  - README to reflect switch profile usage
### Removed
  - Switch-profile subcommand (supplanted with ability to change profile at command line through AWS_PROFILE var)

## 0.3.0 - 2018-05-03
____________________________________________________________________________

### Added
  - Put Item/Delete Item Functions
  - Put basic ```switch-profile``` command in parser/main functionality
### Changed
  - Create Table (Redefined attribute/key schema values)
  - Delete Item subcommand was modified for new key schema in main function
### Removed
  - Unused testing mod (external testing has been implemented)
	- Should be noted that we intend to utilize internal testing when we learn to test against stdout outputs of application

## 0.2.1 - 2018-04-17
____________________________________________________________________________

### Added
  - Create Table (specified region)
  - Delete Table (specified region)
  - Initial Changelog
### Changed
  - ReadMe to reflect new flag usage for create/delete table
  - ReadMe to call out use of SemVer and creation of Changelog
### Removed


## 0.2.0 - 2018-04-03
____________________________________________________________________________

### Added
  - CLAP to parse command line arguments
  - Functionality to create tables (default region)
  - Create Table with specified name
  - Delete Table Functionality
### Changed
  - Moved list_table functionality to library
  - Updated ReadMe to reflect usage, licensing, and switch user ability.
  - Migrated to Mozilla Public License
  - Lint checks added to Travis CI Test
### Removed
  - Initial hash table storing cryptographic hashes from version 0.1.0

## 0.1.0 - 2018-03-20 - [YANKED]
____________________________________________________________________________
  - Initial Release
