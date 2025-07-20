mod models;
mod handlers;
mod utils;
mod menu;

use models::Order;

fn main() {
    let mut orders: Vec<Order> = Vec::new();
    menu::main_menu(&mut orders);
}

