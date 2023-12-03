mod pizza;

use crate::pizza::Pizza;

fn main() {
    let toppings = vec![String::from("topping1"), String::from("topping 2")];
    let pizza = Pizza::new(String::from("my pizza"),toppings,10);
    println!("my pizza is {:?}", pizza);
}