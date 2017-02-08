use hardware_io::{OrderType, N_FLOORS};

#[derive(PartialEq)]
enum ServiceDirection {
    UP,
    DOWN,
    IDLE,
}

struct ElevatorData {
    up_orders: [bool; N_FLOORS as usize],
    down_orders: [bool; N_FLOORS as usize],
    cab_orders: [bool; N_FLOORS as usize],
    floor: i32,
    Direction: ServiceDirection,
}

impl ElevatorData {
    pub fn set_order(&mut self, order_type: OrderType, floor: i32, value: bool) {
        match order_type {
            UpOrder => self.up_orders[floor as usize] = true,
            DownOrder => self.down_orders[floor as usize] = true,
            CabOrder => self.cab_orders[floor as usize] = true,
        }
    }

    pub fn update_state(&mut self, new_floor: i32, new_direction: ServiceDirection) {
        self.floor = new_floor;
        self.Direction = new_direction;
    }
    pub fn get_order_cost(&self, floor: i32, order_type: OrderType) -> i32 {
        let distance = (self.floor - floor).abs();
        let toward_order = self.floor < floor && self.Direction == ServiceDirection::UP ||
                           self.floor > floor && self.Direction == ServiceDirection::DOWN;
        let same_direction =
            (order_type == OrderType::UP && self.Direction == ServiceDirection::UP) ||
            (order_type == OrderType::DOWN && self.Direction == ServiceDirection::UP);

        if self.Direction == ServiceDirection::IDLE {
            return (self.floor - floor).abs();
        } else if !toward_order {
            return N_FLOORS - 1;
        } else if same_direction {
            return distance - 1;
        } else {
            return distance;
        }
    }
    pub fn get_local_command(){
        unimplemented!();
    }
    
    //TODO: Merge into get_local_command
    fn get_new_service_direction(&self) -> ServiceDirection {
        if self.Direction == ServiceDirection::DOWN && self.SearchUnder(self.floor) {
		    return ServiceDirection::DOWN;
	    }
	    else if self.Direction == ServiceDirection::UP && self.SearchOver(self.floor) {
	    	return ServiceDirection::UP;
	    }
	    else if self.Direction == ServiceDirection::UP && self.SearchUnder(self.floor) {
	    	return ServiceDirection::DOWN;
	    }	
	    else if self.Direction == ServiceDirection::DOWN && self.SearchOver(self.floor) {
	    	return ServiceDirection::UP;
	    } else { 
            return ServiceDirection::IDLE;
        }
    }

    //TODO: Merge into get_local_command
    fn get_order_in_floor(&self, direction: OrderType) -> bool {
	if direction == OrderType::DOWN {
		if self.cab_orders[self.floor as usize] == true 
        || self.down_orders[self.floor as usize] == true 
        || self.floor == 0 
        || self.SearchUnder(self.floor - 1) == false {
			return true;
		}
	}
	else if direction == OrderType::UP {
		if self.cab_orders[self.floor as usize] == true || self.up_orders[self.floor as usize] == true || self.floor == N_FLOORS- 1 || self.SearchOver(self.floor + 1) == false {
			return true;
		}
	}
	return false;
    }


    fn SearchOver(&self,floor: i32) -> bool
    {
	let mut i = floor as usize;
	while i < N_FLOORS as usize {
		if self.up_orders[i] == true || self.cab_orders[i] == true || self.down_orders[i] == true {
			return true;
		}
		i += 1;
	}
	return false;
    }

    fn SearchUnder(&self,floor: i32) -> bool
    {
	let mut i = floor;
	while i >= 0 {
		if self.up_orders[i as usize] == true || self.cab_orders[i as usize] == true || self.down_orders[i as usize] == true {
			return true;
		}
		i -= 1;
	}
	return false;
}
}
