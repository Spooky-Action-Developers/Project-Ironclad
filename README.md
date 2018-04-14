
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

## Description

Project Ironclad is a command line utility to effectively create, store and retrieve secrets (or credentials) through Amazon Web Services (AWS). The program utilizes the Rust programming language. In particular, it uses the Rust-Nightly branch of the Rust Language project and is built on top of Rusoto, a AWS SDK that utilizes the AWS API.

## Usage

**_NOTE_**: We are currently in the early stages of learning to implement Rusoto in the project and, so, usage has yet to be fulfilled. As of now, the project is a set of small test functions to establish that we have server connectivity and the ability to handle data inside of the DynamoDB tables stored through AWS.

## License

Project Ironclad is distributed under the terms of the Mozilla Public License.
See [License](LICENSE) for details.
