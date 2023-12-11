use actix_web::{HttpResponse, get, http::header::LOCATION};

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

#[get("/callback")]
async fn get_callback() -> HttpResponse {
    HttpResponse::Ok().body("this is /callback")
}