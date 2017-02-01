mod driver;

pub enum SendMessageCommand {
    IAmStuck,
    OrderComplete{type: driver::elev_button_type_t, floor: i32},
    StateUpdate{direction: driver::elev_button_type_t, floor: i32},
    NewOrder{type: driver::elev_button_ type_t, floor: i32, id: usize}, // usize for use in array indexing, other types might be more appropriate
}