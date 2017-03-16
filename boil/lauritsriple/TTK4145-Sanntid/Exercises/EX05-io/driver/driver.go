package driver

import (
	"time"
	"log"
)

const N_FLOORS = 4
const N_LIGHTS = 4
const N_BUTTONS = 3

//Milliseconds between each polling round
const POLLRATE = 20*time.Millisecond

type MotorDirection int
const (
	MD_up = 1
	MD_down = -1
	MD_stop = 0
)
type BtnEvent struct{
	Floor int
	Button int
}

var driverInitialized=false

func Driver_init() bool{
	if driverInitialized{
		log.Fatal("ERROR, driver already initialized")
	} else {
		driverInitialized=true
		if io_init()==false {
			log.Fatal("ERROR, could not initialize driver")
		} else {
			//sucess
			return true
		}
	}
	return false
}

//LIGHTS: FLOOR INDICATORS
var FloorIndicatorChannels = [N_FLOORS] int {
	LIGHT_FLOOR_IND1,
	LIGHT_FLOOR_IND2,
}

func Driver_setFloorIndicator(floor int){
	switch floor{
	case 1:
		io_clearBit(LIGHT_FLOOR_IND1)
		io_clearBit(LIGHT_FLOOR_IND2)
	case 2:
		io_clearBit(LIGHT_FLOOR_IND1)
		io_setBit(LIGHT_FLOOR_IND2)
	case 3:
		io_setBit(LIGHT_FLOOR_IND1)
		io_clearBit(LIGHT_FLOOR_IND2)
	case 4:
		io_setBit(LIGHT_FLOOR_IND1)
		io_setBit(LIGHT_FLOOR_IND2)

	}
}

//LIGHTS: UP;DOWN;COMMAND
var lightChannels = [N_FLOORS][N_LIGHTS] int {
	{LIGHT_UP1,0,LIGHT_COMMAND1},
	{LIGHT_UP2,LIGHT_DOWN2,LIGHT_COMMAND2},
	{LIGHT_UP3,LIGHT_DOWN3,LIGHT_COMMAND3},
	{0,LIGHT_DOWN4,LIGHT_COMMAND4},
}

func Driver_setBtnLight(floor int, btn int, val bool){
	if val{
		io_setBit(lightChannels[floor][btn])
	} else{
		io_clearBit(lightChannels[floor][btn])
	}
}

func Driver_setStopLight(val bool){
	if val{
		io_setBit(LIGHT_STOP)
	} else {
		io_clearBit(LIGHT_STOP)
	}
}

func Driver_setDoorLight(val bool){
	if val{
		io_setBit(LIGHT_DOOR_OPEN)
	} else {
		io_clearBit(LIGHT_DOOR_OPEN)
	}
}

//BUTTONS: UP;DOWN;COMMAND
var btnChannels = [N_FLOORS][N_BUTTONS] int {
	{BUTTON_UP1,0,BUTTON_COMMAND1},
	{BUTTON_UP2,BUTTON_DOWN2,BUTTON_COMMAND2},
	{BUTTON_UP3,BUTTON_DOWN3,BUTTON_COMMAND3},
	{0,BUTTON_DOWN4,BUTTON_COMMAND4},
}

func Driver_btnPoller(recv chan <- BtnEvent){
	var prev [N_FLOORS][N_BUTTONS] int

	for {
		time.Sleep(POLLRATE)
		for f:=0; f<N_FLOORS; f++{
			for b:=0; b<N_BUTTONS; b++{
				curr:=io_readBit(btnChannels[f][b])
				if (curr != 0 && curr != prev[f][b]){
					recv <- BtnEvent{f,b}
					prev[f][b]=curr
				}
			}
		}
	}
}

func Driver_btnStopPoller(recv chan <- int){
	var prev int
	for{
		time.Sleep(POLLRATE)
		curr:=io_readBit(STOP)
		if (curr!=0 && curr!=prev){
			recv <- curr
			prev=curr
		}
	}
}

func Driver_obstructionPoller(recv chan <- int){
	var prev int
	for{
		time.Sleep(POLLRATE)
		curr:=io_readBit(OBSTRUCTION)
		if (curr!=0 && curr!=prev){
			recv <- curr
			prev=curr
		}
	}
}

//FLOORSENSORS
var floorSensorChannels = [N_FLOORS] int {
	SENSOR_FLOOR1,
	SENSOR_FLOOR2,
	SENSOR_FLOOR3,
	SENSOR_FLOOR4,
}

func Driver_floorSensorPoller(recv chan <- int){
	var prev int
	for{
		time.Sleep(POLLRATE)
		for f:=0; f<N_FLOORS; f++{
			curr:=io_readBit(floorSensorChannels[f])
			if (curr!=0 && f!=prev){
				recv <- f
				prev=f
			}
		}
	}
}

func Driver_setMotorDir(dir MotorDirection){
	switch dir{
	case MD_stop:
		io_writeAnalog(MOTOR,0)
	case MD_up:
		io_clearBit(MOTORDIR)
		io_writeAnalog(MOTOR,2800)
	case MD_down:
		io_setBit(MOTORDIR)
		io_writeAnalog(MOTOR,2800)
	}
}
