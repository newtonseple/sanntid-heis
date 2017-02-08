use hardware_io::{OrderType, N_FLOORS};

enum ServiceDirection {
    UP,
    DOWN,
    IDLE,
}

struct ElevatorData {
    UpOrders: [bool; N_FLOORS as usize],
    DownOrders: [bool; N_FLOORS as usize],
    CabOrders: [bool; N_FLOORS as usize],
    Floor: i32,
    Direction: ServiceDirection,
}

impl ElevatorData {
    pub fn set_order(&mut self, order_type: OrderType, floor: i32, value: bool) {
        match order_type {
            UpOrder => self.UpOrders[floor as usize] = true,
            DownOrder => self.DownOrders[floor as usize] = true,
            CabOrder => self.CabOrders[floor as usize] = true,
        }
    }

    pub fn update_state(&mut self, new_floor: i32, new_direction: MotorDirection) {
        self.Floor = new_floor;
        self.Direction = new_direction;
    }

    pub fn get_new_service_direction(&self, state: ElevatorState) -> ServiceDirection {
        if (self.Direction == ServiceDirection::DOWN && SearchUnder(floor)) {
		    return ServiceDirection::DOWN;
	    }
	    else if (self.Direction == ServiceDirection::UP && SearchOver(floor)) {
	    	return ServiceDirection::UP;
	    }
	    else if (self.Direction == ServiceDirection::UP && SearchUnder(floor)) {
	    	return ServiceDirection::DOWN;
	    }	
	    else if (self.Direction == ServiceDirection::DOWN && SearchOver(floor)) {
	    	return ServiceDirection::UP;
	    } else { 
            return ServiceDirection::IDLE;
        }
    }

    pub fn get_order_cost(&self, floor: i32, order_type: OrderType) -> i32 {
        let distance = (self.Floor - floor).abs();
        let toward_order = self.Floor < floor && self.Direction == ServiceDirection::UP ||
                           self.Floor > floor && self.Direction == ServiceDirection::DOWN;
        let same_direction =
            (order_type == OrderType::UP && self.Direction == ServiceDirection::UP) ||
            (order_type == OrderType::DOWN && self.Direction == ServiceDirection::UP);

        if self.Direction == ServiceDirection::IDLE {
            return (self.Floor - floor).abs();
        } else if !toward_order {
            return N_FLOORS - 1;
        } else if same_direction {
            return distance - 1;
        } else {
            return distance;
        }
    }

    pub fn get_order_in_floor(floor: i32, elev_motor_direction_t direction: OrderType) -> bool {
	if (direction == OrderType::DOWN) {
		if (self.CabOrders[floor] == true || self.DownOrders[floor] == true || floor == 0 || self.SearchUnder(floor - 1) == false) {
			return true;
		}
	}
	else if (direction == OrderType::UP) {
		if (self.CabOrders[floor] == true || self.UpOrders[floor] == true || floor == N_FLOORS - 1 || self.SearchOver(floor + 1) == false) {
			return true;
		}
	}
	return false;
    }


    fn SearchOver(floor: i32) -> bool
    {
	let mut i = floor;
	while (i < N_FLOORS) {
		if (self.UpOrders[i] == true || self.CabOrders[i] == true || self.DownOrders[i] == true) {
			return true;
		}
		i += 1;
	}
	return false;
    }

    fn SearchUnder(floor: i32) -> bool
    {
	let mut i = floor;
	while (i >= 0) {
		if (self.UpOrders[i] == true || self.CabOrders[i] == true || self.DownOrders[i] == true) {
			return true;
		}
		i -= 1;
	}
	return false;
}
}
