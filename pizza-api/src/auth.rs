use actix_web::{dev::ServiceRequest, Error, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use reqwest::header::USER_AGENT;

pub async fn validate(request: ServiceRequest, bearer_auth: BearerAuth)
-> Result<ServiceRequest, (Error, ServiceRequest)> {

    let token = bearer_auth.token();

    let client = reqwest::Client::new();
    let response = client
    .get("https://api.github.com/user")
    .bearer_auth(token)
    .header(USER_AGENT, "pizza-api-validator")
    .send()
    .await;

    match response {
        Ok(result) if result.status().is_success() => {
            Ok(request)
        }
        _ => Err((ErrorUnauthorized("could not validate the token"), request)),
    }
}