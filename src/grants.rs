use rusoto_core::region::Region;
use rusoto_kms::*;



pub fn create_grant_request(grantee_principal: &str, key_id: &str, retiring_principal: &str, grant_name: &str)
                        ->Result<CreateGrantRequest,CreateGrantError>{
    let mut grant_request = rusoto_kms::CreateGrantRequest::default();
    let mut operations = vec!["Decrypt".to_string(), "Encrypt".to_string()];
    grant_request.operations = operations;
    grant_request.grantee_principal = grantee_principal.to_string();
    grant_request.key_id= key_id.to_string();
    grant_request.name = Some(grant_name.to_string());
    grant_request.retiring_principal = Some(retiring_principal.to_string());
    Ok(grant_request)
}

//Function call examples:
/*let mut new_gr_req = create_grant_request("ARN of recipient", "Key ID of CMK",
                "ARN of account that will retire grant", "Nickname your Grant").unwrap();
*/

/*let mut grant_token = kms_client.create_grant(&new_gr_req).sync().unwrap();
println!("Token ID is: {:?}", grant_token.grant_id);*/



//Retire Access grant
pub fn create_retire_req(grant_id: &str,  key_id: &str)-> Result<RetireGrantRequest,RetireGrantError>{
    let mut retire_req = RetireGrantRequest::default();
    retire_req.grant_id = Some(grant_id.to_string());
    retire_req.key_id =Some(key_id.to_string());
    Ok(retire_req)
}

//Function call examples:
/*let mut new_retire = create_retire_req("Grant ID",
                                    "Key ID of CMK").unwrap();
let kms_client = KmsClient::simple(Region::default());
kms_client.retire_grant(&new_retire).sync();*/

//List Grants
pub fn list_grants_request(key_id: &str)-> Result<ListGrantsRequest,ListGrantsError> {
    let mut list_request = rusoto_kms::ListGrantsRequest::default();
    list_request.key_id = key_id.to_string();
    Ok(list_request)
}

//Function call examples:
/*let mut list_req =list_grants_request("Key ID of CMK").unwrap();
let mut list = kms_client.list_grants(&list_req).sync().unwrap();
let mut grants =list.grants.unwrap();*/

//println!("This is your list of grants: {:?}",grants);


//Revoke Access grant request
pub fn revoke_grant_req(grant_id: &str, key_id: &str)->Result<RevokeGrantRequest,RevokeGrantError>{
    let mut revoke_request = rusoto_kms::RevokeGrantRequest::default();
    revoke_request.grant_id = grant_id.to_string();
    revoke_request.key_id = key_id.to_string();
    Ok(revoke_request)
}

//Function call examples:
/*let revoke_req = revoke_grant_req("Grant ID",
                                  "Key ID of CMK").unwrap();
let kms_client = KmsClient::simple(Region::default());
kms_client.revoke_grant(&revoke_req);*/
