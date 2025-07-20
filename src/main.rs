mod models;
mod handlers;
mod utils;
mod menu;
use uuid::Uuid;
use models::Order;
use std::collections::HashMap;

// fn main() {
//     let mut orders: Vec<Order> = Vec::new();
//     menu::main_menu(&mut orders);
// }




fn main() {
    let mut orders: HashMap<Uuid, Order> = HashMap::new();
    menu::main_menu(&mut orders);
}


