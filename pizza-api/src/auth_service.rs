use actix_web::{HttpResponse, get};

#[get("/sign_in")]
async fn get_sign_in() -> HttpResponse {
    HttpResponse::Ok().body("this is /sign_in")
}

#[get("/callback")]
async fn get_callback() -> HttpResponse {
    HttpResponse::Ok().body("this is /callback")
}