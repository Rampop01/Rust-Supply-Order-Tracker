use crate::handlers::{add_order, view_orders};
use crate::models::Order;
use crate::utils::get_input;

pub fn main_menu(orders: &mut Vec<Order>) {
    loop {
        println!("\n=== Supply Order Tracker ===");
        println!("1. Add Order");
        println!("2. View Orders");
        println!("3. Exit");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => add_order(orders),
            "2" => view_orders(orders),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
