use hardware_io::{OrderType, N_FLOORS};
use local_controller::LocalCommandMessage;
use super::Order;

#[derive(Debug, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ServiceDirection {
    UP,
    DOWN,
    IDLE,
}

#[derive(Debug)]
pub struct ElevatorData {
    up_orders: [bool; N_FLOORS as usize],
    down_orders: [bool; N_FLOORS as usize],
    cab_orders: [bool; N_FLOORS as usize],
    floor: i32,
    direction: ServiceDirection,
    pub alive: bool, //Note: This can safely be public. A setter/getter would in this case _only_ add complexity, and would not further minimize accessibility to the data members.
}

impl ElevatorData {
    pub fn new() -> ElevatorData {
        ElevatorData {
            up_orders: [false; N_FLOORS as usize],
            down_orders: [false; N_FLOORS as usize],
            cab_orders: [false; N_FLOORS as usize],
            floor: 0,
            direction: ServiceDirection::IDLE,
            alive: true,
        }
    }
    pub fn set_order(&mut self, order_type: OrderType, floor: i32, value: bool) {
        match order_type {
            OrderType::UP => self.up_orders[floor as usize] = value,
            OrderType::DOWN => self.down_orders[floor as usize] = value,
            OrderType::CAB => self.cab_orders[floor as usize] = value,
        }
    }
    pub fn get_orders(&self) -> Vec<Order> {
        let up_order_iter = self.up_orders.iter().enumerate().filter_map(|(floor, order_value)| -> Option<Order> {
            if *order_value {
                Some( Order {
                    order_type: OrderType::UP,
                    floor: floor as i32,
                })
            } else {
                None
            }  
        });
        let down_order_iter = self.down_orders.iter().enumerate().filter_map(|(floor, order_value)| -> Option<Order> {
            if *order_value {
                Some( Order {
                    order_type: OrderType::DOWN,
                    floor: floor as i32,
                })
            } else {
                None
            }  
        });
        let cab_order_iter = self.cab_orders.iter().enumerate().filter_map(|(floor, order_value)| -> Option<Order> {
            if *order_value {
                Some( Order {
                    order_type: OrderType::CAB,
                    floor: floor as i32,
                })
            } else {
                None
            }  
        });
        
        up_order_iter.chain(down_order_iter.chain(cab_order_iter)).collect()
    }

    pub fn update_state(&mut self, new_floor: i32, new_direction: ServiceDirection) {
        self.floor = new_floor;
        self.direction = new_direction;
    }

    pub fn get_order_cost(&self, floor: i32, order_type: OrderType) -> i32 {
        let distance = (self.floor - floor).abs();
        let toward_order = self.floor < floor && self.direction == ServiceDirection::UP ||
                           self.floor > floor && self.direction == ServiceDirection::DOWN;
        let same_direction =
            (order_type == OrderType::UP && self.direction == ServiceDirection::UP) ||
            (order_type == OrderType::DOWN && self.direction == ServiceDirection::UP);

        if self.direction == ServiceDirection::IDLE {
            return (self.floor - floor).abs();
        } else if !toward_order {
            return N_FLOORS - 1;
        } else if same_direction {
            return distance - 1;
        } else {
            return distance;
        }
    }

    pub fn get_local_command(&self) -> LocalCommandMessage {
        if self.direction == ServiceDirection::DOWN {
            if self.cab_orders[self.floor as usize] == true ||
               self.down_orders[self.floor as usize] == true {
                return LocalCommandMessage::StopForOrder{order_type: OrderType::DOWN}
            } else if self.search_below(self.floor - 1) == true {
                return LocalCommandMessage::GoDown;
            } else if self.up_orders[self.floor as usize] == true {
                return LocalCommandMessage::StopForOrder{order_type: OrderType::UP}
            } else {
                return LocalCommandMessage::DoNothing;
            }
        } else if self.direction == ServiceDirection::UP {
            if self.cab_orders[self.floor as usize] == true ||
               self.up_orders[self.floor as usize] == true {
                return LocalCommandMessage::StopForOrder{order_type: OrderType::UP}
            } else if self.search_above(self.floor + 1) == true {
                return LocalCommandMessage::GoUp;
            } else if self.down_orders[self.floor as usize] == true {
                return LocalCommandMessage::StopForOrder{order_type: OrderType::DOWN}
            } else {
                return LocalCommandMessage::DoNothing
            }
        } else { // Lift is idle
            if self.cab_orders[self.floor as usize] == true ||
               self.down_orders[self.floor as usize] == true {
                return LocalCommandMessage::StopForOrder{order_type: OrderType::DOWN}
            } else if self.up_orders[self.floor as usize] == true {
                return LocalCommandMessage::StopForOrder{order_type: OrderType::UP}
            } else if self.search_below(self.floor - 1) == true {
                return LocalCommandMessage::GoDown;
            } else if self.search_above(self.floor + 1) == true {
                return LocalCommandMessage::GoUp;
            } else {
                return LocalCommandMessage::DoNothing;
            }
        }
    }

    fn search_above(&self, floor: i32) -> bool {
        let mut i = floor as usize;
        while i < N_FLOORS as usize {
            if self.up_orders[i] == true || self.cab_orders[i] == true ||
               self.down_orders[i] == true {
                return true;
            }
            i += 1;
        }
        return false;
    }

    fn search_below(&self, floor: i32) -> bool {
        let mut i = floor;
        while i >= 0 {
            if self.up_orders[i as usize] == true || self.cab_orders[i as usize] == true ||
               self.down_orders[i as usize] == true {
                return true;
            }
            i -= 1;
        }
        return false;
    }
}
