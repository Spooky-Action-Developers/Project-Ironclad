use std::collections::HashMap;
use std::collections::HashSet;
extern crate crypto;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

fn main() {
    let mut hash_classes = HashMap::new();
    hash_classes.insert(String::from("SHA1"),       1);
    hash_classes.insert(String::from("SHA224"),     2);
    hash_classes.insert(String::from("SHA256"),     3);
    hash_classes.insert(String::from("SHA384"),     4);
    hash_classes.insert(String::from("SHA512"),     5);
    hash_classes.insert(String::from("RIPEMD"),     6);
    hash_classes.insert(String::from("WHIRLPOOL"),  7);
    hash_classes.insert(String::from("MD5"),        8);

    static DEFAULT_DIGEST: &str = "SHA256";
    println!("{}",DEFAULT_DIGEST);
    //const HASHING_ALGOS: HashSet<String> = hash_classes.keys();
}
