# Project Ironclad Changelog

All notable changes to the project will be documented here.

## 0.3.2 - 2018-05-14
____________________________________________________________________________

## Added
  - New ```put``` functionality to store with a 
  - Added KMS to encrypt and decrypt secrets on ```put``` and ```get```
## Changed
  - Main functionality updated for put subcommand to take secret values
  - Updated ```put_item``` function to add secret attribute and store it to AWS
##Removed
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
