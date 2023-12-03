#[derive(Debug)]
pub struct Pizza {
    name: String,
    toppings: Vec<String>,
    price: i32,
}

impl Pizza {
    pub fn new(name: String, toppings: Vec<String>, price: i32) -> Pizza {
        let pizza = Pizza {
            name,
            toppings,
            price
        };
        return pizza
    }
}
