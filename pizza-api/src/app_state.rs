use crate::pizza::Pizza;

pub struct AppState {
    pizza: Vec<Pizza>
}

impl AppState {
    pub fn new(pizza: Vec<Pizza>) -> Self {
        AppState {
            pizza
        }
    }
    pub fn get_list_pizza(&self) -> &Vec<Pizza> {
        &self.pizza
    }
}