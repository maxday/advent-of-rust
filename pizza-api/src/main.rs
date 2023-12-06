mod pizza;

use crate::pizza::Pizza;
use std::env::{self, VarError};

fn get_port(result: Result<String, VarError>) -> i32 {
    let default_port = 8000;
    match result {
        Ok(number) => number.parse::<i32>().unwrap_or(default_port),
        Err(_) => default_port
    }
}
fn main() {
    // let toppings = vec![String::from("topping1"), String::from("topping 2")];
    // let pizza = Pizza::new(String::from("my pizza"),toppings,10);
    // println!("my pizza is {:?}", pizza);

    
    println!("real port is {}", get_port(std::env::var("PORT")));
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