use crate::handlers::{add_order, view_orders, remove_order,edit_order,fulfill_order};
use crate::models::Order;
use crate::utils::get_input;
use std::collections::HashMap;
use uuid::Uuid;

pub fn main_menu(orders: &mut HashMap<Uuid, Order>) {
    loop {
        println!("\n=== Supply Order Tracker ===");
        println!("1. Add Order");
        println!("2. View Orders");
        println!("3. Remove Order");
        println!("4. Edit Order");
        println!("5. Fulfill Order");
        println!("6. Exit");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => add_order(orders),
            "2" => view_orders(orders),
            "3" => remove_order(orders),
            "4" => edit_order(orders),
            "5" => fulfill_order(orders),
            "6" => {
                println!("See you next time");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
