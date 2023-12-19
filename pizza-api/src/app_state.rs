use std::sync::Mutex;

use crate::pizza::Pizza;

pub struct AppState {
    pizza: Mutex<Vec<Pizza>>
}

impl AppState {
    pub fn new() -> Self {
        let pizza_db = Vec::<Pizza>::new();
        AppState {
            pizza: Mutex::new(pizza_db)
        }
    }
    pub fn get_list_pizza(&self) -> Vec<Pizza> {
        match self.pizza.lock() {
            Ok(pizza) => pizza.to_vec(),
            Err(_) => Vec::<Pizza>::default()
        }
    }
    pub fn add_pizza(&self, new_pizza: Pizza) -> Result<(), String> {
        let mut pizza = self.pizza.lock().map_err(|e| e.to_string())?;
        pizza.push(new_pizza);
        Ok(())
    }
}