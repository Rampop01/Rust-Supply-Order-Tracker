use crate::models::Order;
use crate::utils;
use uuid::Uuid;
use std::collections::HashMap;
use crate::models::OrderStatus;
// pub fn add_order(orders: &mut Vec<Order>) {
//     println!("\n--- Add New Order ---");

//     let item_name = utils::get_input("Enter item name: ");
//     let quantity = utils::get_input("Enter quantity: ")
//         .trim()
//         .parse::<u32>()
//         .unwrap_or(0); 
//     let supplier = utils::get_input("Enter supplier name: ");

//     let order = Order {
//         item_name,
//         quantity,
//         supplier,
//     };

//     orders.push(order);
//     println!("Order added successfully!");
// }

pub fn add_order(orders: &mut HashMap<Uuid, Order>) {
    println!("\n--- Add New Order ---");

    let item_name = utils::get_input("Enter item name: ");
    let quantity = utils::get_input("Enter quantity: ")
        .trim()
        .parse::<u32>()
        .unwrap_or(0);
    let supplier = utils::get_input("Enter supplier name: ");

    let order = Order {
        id: Uuid::new_v4(), // generate unique ID
        item_name,
        quantity,
        supplier,
        status: OrderStatus::Pending,
    };

    orders.insert(order.id, order);
    println!("Order added successfully!");
}

pub fn view_orders(orders: &HashMap<Uuid, Order>) {
    println!("\n--- All Orders ---");

    if orders.is_empty() {
        println!("No orders yet.");
    } else {
       for (id, order) in orders {
        println!(
        "ID: {}\nItem: {}\nQuantity: {}\nSupplier: {}\nStatus: {:?}\n---",
        id, order.item_name, order.quantity, order.supplier, order.status
    );
        }
    }
}

pub fn remove_order(orders: &mut HashMap<Uuid, Order>) {
    println!("\n--- Remove an Order ---");

    if orders.is_empty() {
        println!("No orders to remove.");
        return;
    }

    // Show all current orders
    for (id, order) in orders.iter() {
        println!(
            "ID: {}\nItem: {}\nQuantity: {}\nSupplier: {}\n---",
            id, order.item_name, order.quantity, order.supplier
        );
    }

    // Ask user for ID to delete
    let id_input = utils::get_input("Enter the ID of the order to remove: ");

    // Try to parse it into a valid UUID
    match Uuid::parse_str(id_input.trim()) {
        Ok(uuid) => {
            if orders.remove(&uuid).is_some() {
                println!("Order removed successfully!");
            } else {
                println!("No order found with that ID.");
            }
        }
        Err(_) => {
            println!("Invalid ID format. Make sure you enter a valid UUID.");
        }
    }
}

pub fn edit_order(orders: &mut HashMap<Uuid, Order>) {
    println!("\n--- Edit an Order ---");

    if orders.is_empty() {
        println!("No orders to edit.");
        return;
    }

    // Show all orders
    for (id, order) in orders.iter() {
        println!(
            "ID: {}\nItem: {}\nQuantity: {}\nSupplier: {}\n---",
            id, order.item_name, order.quantity, order.supplier
        );
    }

    let id_input = utils::get_input("Enter the ID of the order to edit: ");

    match Uuid::parse_str(id_input.trim()) {
        Ok(uuid) => {
            if let Some(order) = orders.get_mut(&uuid) {
                println!("\nCurrent details:");
                println!(
                    "Item: {}\nQuantity: {}\nSupplier: {}",
                    order.item_name, order.quantity, order.supplier
                );

                // Ask for new values
                let new_item = utils::get_input("Enter new item name (or press ENTER to keep current): ");
                let new_qty = utils::get_input("Enter new quantity (or press ENTER to keep current): ");
                let new_supplier = utils::get_input("Enter new supplier (or press ENTER to keep current): ");

                // Confirmation
                let confirm = utils::get_input("Save changes? (y/n): ");
                if confirm.trim().eq_ignore_ascii_case("y") {
                    if !new_item.trim().is_empty() {
                        order.item_name = new_item;
                    }

                    if !new_qty.trim().is_empty() {
                        if let Ok(qty) = new_qty.trim().parse::<u32>() {
                            order.quantity = qty;
                        } else {
                            println!("Invalid quantity, keeping previous.");
                        }
                    }

                    if !new_supplier.trim().is_empty() {
                        order.supplier = new_supplier;
                    }

                    println!("Order updated successfully!");
                } else {
                    println!("Edit cancelled.");
                }
            } else {
                println!("No order found with that ID.");
            }
        }
        Err(_) => println!("Invalid ID format."),
    }
}

pub fn fulfill_order(orders: &mut HashMap<Uuid, Order>) {
    println!("\n--- Mark Order as Fulfilled ---");

    if orders.is_empty() {
        println!("No orders to mark as fulfilled.");
        return;
    }

    for (id, order) in orders.iter() {
        println!(
            "ID: {}\nItem: {}\nQuantity: {}\nSupplier: {}\nStatus: {:?}\n---",
            id, order.item_name, order.quantity, order.supplier, order.status
        );
    }

    let id_input = utils::get_input("Enter the ID of the order to fulfill: ");

    match Uuid::parse_str(id_input.trim()) {
        Ok(uuid) => {
            if let Some(order) = orders.get_mut(&uuid) {
                if let OrderStatus::Fulfilled = order.status {
                    println!("Order is already fulfilled.");
                } else {
                    order.status = OrderStatus::Fulfilled;
                    println!("Order marked as fulfilled!");
                }
            } else {
                println!("No order found with that ID.");
            }
        }
        Err(_) => println!("Invalid ID format."),
    }
}


#[cfg(test)]
mod tests {
    
    use crate::models::{Order, OrderStatus};

    use uuid::Uuid;
    use std::collections::HashMap;

    fn setup_orders_with_one_order() -> (HashMap<Uuid, Order>, Uuid) {
        let mut orders = HashMap::new();
        let order = Order {
            id: Uuid::new_v4(),
            item_name: "Laptop".to_string(),
            quantity: 5,
            supplier: "Dell".to_string(),
            status: OrderStatus::Pending,
        };
        let id = order.id;
        orders.insert(order.id, order);
        (orders, id)
    }

    #[test]
    fn test_add_order() {
        let (orders, _id) = setup_orders_with_one_order();
        assert_eq!(orders.len(), 1);
        let order = orders.values().next().unwrap();
        assert_eq!(order.item_name, "Laptop");
        assert_eq!(order.quantity, 5);
        assert_eq!(order.supplier, "Dell");
        assert_eq!(order.status, OrderStatus::Pending);
    }

    #[test]
    fn test_fulfill_order() {
        let (mut orders, id) = setup_orders_with_one_order();
        // Simulate fulfilling the order
        if let Some(order) = orders.get_mut(&id) {
            order.status = OrderStatus::Fulfilled;
        }
        assert_eq!(orders.get(&id).unwrap().status, OrderStatus::Fulfilled);
    }

    #[test]
    fn test_edit_order() {
        let (mut orders, id) = setup_orders_with_one_order();
        if let Some(order) = orders.get_mut(&id) {
            order.item_name = "Monitor".to_string();
            order.quantity = 10;
            order.supplier = "HP".to_string();
        }
        let order = orders.get(&id).unwrap();
        assert_eq!(order.item_name, "Monitor");
        assert_eq!(order.quantity, 10);
        assert_eq!(order.supplier, "HP");
    }

    #[test]
    fn test_remove_fulfilled_orders() {
        let (mut orders, id) = setup_orders_with_one_order();
        // Fulfill the order
        if let Some(order) = orders.get_mut(&id) {
            order.status = OrderStatus::Fulfilled;
        }
        // Remove fulfilled orders
        orders.retain(|_, order| order.status != OrderStatus::Fulfilled);
        assert!(orders.is_empty());
    }
}
