mod pizza;

use crate::pizza::Pizza;
use std::env;

fn get_port() -> String {
    let port = env::var("PORT");
    match port {
        Ok(number) => number,
        Err(e) => String::from("8000")
    }
}
fn main() {
    // let toppings = vec![String::from("topping1"), String::from("topping 2")];
    // let pizza = Pizza::new(String::from("my pizza"),toppings,10);
    // println!("my pizza is {:?}", pizza);

    
    println!("real port is {}", get_port());
}