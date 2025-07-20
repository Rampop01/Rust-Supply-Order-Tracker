// #[derive(Debug)]
// pub struct Order {
//     pub item_name: String,
//     pub quantity: u32,
//     pub supplier: String,
// }
use uuid::Uuid;
#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Fulfilled,
}

#[derive(Debug)]
pub struct Order {
    pub id: Uuid,           
    pub item_name: String,
    pub quantity: u32,
    pub supplier: String,
    pub status: OrderStatus,
}
