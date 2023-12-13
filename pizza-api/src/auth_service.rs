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
    exchange_code_for_token(&req.code).await;
    HttpResponse::Ok().body("this is /callback")
}

async fn exchange_code_for_token(code: &str) -> String {
    // this is bad
    let url = std::env::var("GITHUB_TOKEN_URL").unwrap();
    let client_id = std::env::var("GITHUB_CLIENT_ID").unwrap();
    let client_secret_id = std::env::var("GITHUB_CLIENT_SECRET_ID").unwrap();
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
    .unwrap();

    println!("response = {:?}", response.text().await);

    return String::from("my-token");
}