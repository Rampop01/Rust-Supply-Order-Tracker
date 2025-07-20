use crate::models::Order;
use crate::utils;

pub fn add_order(orders: &mut Vec<Order>) {
    println!("\n--- Add New Order ---");

    let item_name = utils::get_input("Enter item name: ");
    let quantity = utils::get_input("Enter quantity: ")
        .trim()
        .parse::<u32>()
        .unwrap_or(0); 
    let supplier = utils::get_input("Enter supplier name: ");

    let order = Order {
        item_name,
        quantity,
        supplier,
    };

    orders.push(order);
    println!("Order added successfully!");
}

pub fn view_orders(orders: &Vec<Order>) {
    println!("\n--- All Orders ---");

    if orders.is_empty() {
        println!("No orders yet.");
    } else {
        for (i, order) in orders.iter().enumerate() {
            println!(
                "{}. Item: {}, Quantity: {}, Supplier: {}",
                i + 1,
                order.item_name,
                order.quantity,
                order.supplier
            );
        }
    }
}
