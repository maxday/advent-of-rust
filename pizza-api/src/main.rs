mod pizza;
mod pizza_service;
mod auth_service;
mod auth;
mod app_state;


use actix_web::{HttpServer, App, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use app_state::AppState;
use auth_service::{get_sign_in, get_callback};
use pizza::Pizza;
use pizza_service::{get_pizza, post_pizza};
use dotenv::dotenv;

use std::env::VarError;

fn get_port(result: Result<String, VarError>) -> u16 {
    let default_port = 8000;
    match result {
        Ok(number) => number.parse::<u16>().unwrap_or(default_port),
        Err(_) => default_port
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    dotenv().ok();
    let pizza_db = Vec::<Pizza>::new();
    let app_state = web::Data::new(AppState::new(pizza_db));
    let http_server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth::validate);
        App::new()
        .app_data(app_state.clone())
        .service(
            web::scope("/auth")
            .service(get_sign_in)
            .service(get_callback)
        )
        .service(
            web::scope("/pizza")
            //.wrap(auth)
            .service(get_pizza)
            .service(post_pizza)
        )
    });
    let binding_info = ("0.0.0.0", get_port(std::env::var("PORT")));
    let http_server = http_server.bind(binding_info)?;
    http_server.run().await
}

#[cfg(test)]
mod test {
    use crate::get_port;

    #[test]
    fn test_valid_port() {
        let result = Ok(String::from("1234"));
        assert_eq!(1234, get_port(result))
    }

    #[test]
    fn test_invalid_port() {
        let result = Ok(String::from("1234-invalid-char"));
        assert_eq!(8000, get_port(result))
    }

    #[test]
    fn test_port_not_set() {
        let result = Err(std::env::VarError::NotPresent);
        assert_eq!(8000, get_port(result))
    }
}