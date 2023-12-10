use actix_web::{dev::ServiceRequest, Error, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validate(request: ServiceRequest, bearer_auth: BearerAuth)
-> Result<ServiceRequest, (Error, ServiceRequest)> {
    let tuple = (ErrorUnauthorized("could not find the token"), request);
    Err(tuple)
    //Ok(request)
}