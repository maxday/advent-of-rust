use std::sync::Mutex;

use crate::pizza::Pizza;

pub struct AppState {
    pizza: Mutex<Vec<Pizza>>
}

impl AppState {
    pub fn new(pizza: Vec<Pizza>) -> Self {
        AppState {
            pizza: Mutex::new(pizza)
        }
    }
    pub fn get_list_pizza(&self) -> Vec<Pizza> {
        match self.pizza.lock() {
            Ok(pizza) => pizza.to_vec(),
            Err(_) => Vec::<Pizza>::default()
        }
    }
    pub fn add_pizza(&self, new_pizza: Pizza) {
        match self.pizza.lock() {
            Ok(mut pizza) => pizza.push(new_pizza),
            Err(_) => {}
        }
    }
}