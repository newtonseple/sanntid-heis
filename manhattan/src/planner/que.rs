use std::vec;

use hardware_io;

struct DelegatedOrder {
    floor: i32,
    responsible: usize, // Index type, other types might be more appropriate
}

pub struct OrderQue {
    UpOrders: [DelegatedOrder; hardware_io::N_FLOORS as usize],
    DownOrders: [DelegatedOrder; hardware_io::N_FLOORS as usize],
    CabOrders: [DelegatedOrder; hardware_io::N_FLOORS as usize],
}

struct ElevatorOrderTable {
    UpOrders: [bool; hardware_io::N_FLOORS as usize],
    DownOrders: [bool; hardware_io::N_FLOORS as usize],
    CabOrders: [bool; hardware_io::N_FLOORS as usize],
}
struct ElevatorState {
    Floor: i32,
    Direction: hardware_io::MotorDirection,
}

impl ElevatorState {
    pub fn getFeasibility(&self, order_type: hardware_io::OrderType) -> i32 {
        unimplemented!();
    }
}

impl ElevatorOrderTable {
    pub fn setOrder(&mut self, order_type: hardware_io::OrderType, floor: i32) {
        match order_type {
            UpOrder => self.UpOrders[floor as usize] = true,
            DownOrder => self.DownOrders[floor as usize] = true,
            CabOrder => self.CabOrders[floor as usize] = true,
        }
    }
}
