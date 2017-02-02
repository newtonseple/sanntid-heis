use std::vec;

mod driver


struct DelegatedOrder {
    floor: i32,
    responsible: usize, // Index type, other types might be more appropriate
}

pub struct OrderQue {
    UpOrders: [DelegatedOrder; driver::N_FLOORS],
    DownOrders: [DelegatedOrder; driver::N_FLOOORS],
    CabOrders: [DelegatedOrder; driver::N_FLOORS],
}

struct AlternativeOrderQue {
    Orders: 
}

impl OrderQue {
     pub fn getIDsOrders(ID: usize) -> Vec<Order> {

     }
}