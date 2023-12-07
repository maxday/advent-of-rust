mod pizza;
mod service;

use actix_web::{HttpServer, App};
use service::get_pizza;

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
    let http_server = HttpServer::new(|| {
        App::new().service(get_pizza)
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