use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pizza {
    name: String,
    toppings: Vec<String>,
    price: i32,
}

impl Pizza {
    pub fn new(name: String, toppings: Vec<String>, price: i32) -> Pizza {
        Pizza {
            name,
            toppings,
            price
        }
    }
}
