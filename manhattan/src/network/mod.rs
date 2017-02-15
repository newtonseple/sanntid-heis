use hardware_io;

pub enum SendMessageCommand {
    IAmStuck,
    OrderComplete {
        order_type: hardware_io::OrderType,
        floor: i32,
    },
    StateUpdate {
        direction: hardware_io::OrderType,
        floor: i32,
    },
    NewOrder {
        order_type: hardware_io::OrderType,
        floor: i32,
        id: usize,
    }, // usize for use in array indexing, other types might be more appropriate
}
