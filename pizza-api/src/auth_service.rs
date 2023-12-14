use std::env::VarError;

use actix_web::{HttpResponse, get, http::header::LOCATION, HttpRequest, web};
use reqwest::{Client, header::ACCEPT};
use serde::Deserialize;

#[get("/sign_in")]
async fn get_sign_in() -> HttpResponse {
    match std::env::var("GITHUB_CLIENT_ID") {
        Ok(client_id) => {
            let url = format!("https://github.com/login/oauth/authorize?client_id={}", client_id);
            HttpResponse::PermanentRedirect().append_header(
                (LOCATION, url)
            ).finish()
        }
        Err(_) => HttpResponse::BadGateway().finish()
    }
}

#[derive(Deserialize)]
struct CodeRequest {
    code: String
}

#[get("/callback")]
async fn get_callback(req: web::Query<CodeRequest>) -> HttpResponse {
    match exchange_code_for_token(&req.code).await {
        Ok(auth_token) => HttpResponse::Ok().body(auth_token.access_token),
        Err(error) => HttpResponse::BadGateway().body(error)
    }
}

#[derive(Deserialize)]
struct AuthToken {
    access_token: String
}

async fn exchange_code_for_token(code: &str) -> Result<AuthToken, String> {
    let url = std::env::var("GITHUB_TOKEN_URL").map_err(|e| e.to_string())?;
    let client_id = std::env::var("GITHUB_CLIENT_ID").map_err(|e| e.to_string())?;
    let client_secret_id = std::env::var("GITHUB_CLIENT_SECRET_ID").map_err(|e| e.to_string())?;
    let payload = [
        ("client_id", client_id),
        ("client_secret", client_secret_id),
        ("code", String::from(code))
    ];

    let client = Client::new();
    // this is bad, we need to refactor tomorrow!
    let response = client
    .post(url)
    .form(&payload)
    .header(ACCEPT, "application/json")
    .send()
    .await
    .map_err(|e| e.to_string())?;

    Ok(response.json::<AuthToken>().await.unwrap())
}