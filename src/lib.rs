extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_dynamodb;
extern crate rusoto_kms;
#[macro_use]
extern crate serde_json;

/*
 * MACROS
 * Macros to be utilized by ironclad system
 * These allow for build patterns of table input
 * and encryption/decryption contexts
 *
 * */

#[macro_export]
macro_rules! attributes {
        ($($val:expr => $attr_type:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push(AttributeDefinition { attribute_name: String::from($val),
                                    attribute_type: String::from($attr_type) });
                )*
                temp_vec
            }
        }
    }

#[macro_export]
macro_rules! key_schema {
        ($($name:expr => $key_type:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push(KeySchemaElement { key_type: String::from($key_type),
                                    attribute_name: String::from($name) });
                )*
                temp_vec
            }
        }
    }
#[macro_export]
macro_rules! val {
    (B => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.b = Some($val);
        attr
    }};
    (S => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.s = Some($val.to_string());
        attr
    }};
    (N => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.n = Some($val.to_string());
        attr
    }};
}

/*
 * MODS
 * The listing of mods used by ironclad system:
 */

pub mod secrets;
pub mod tables;
