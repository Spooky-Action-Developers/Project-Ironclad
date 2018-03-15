extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;

pub mod tables;
pub mod profiles;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_authenticate_user(){
        // Set AWS Variables to NULL values originally

        // Run Authenticate User method

        // Assert equality of AWS environment
        // vars with their expected values
    }

    fn t_store_secret(){
        
    }
}
