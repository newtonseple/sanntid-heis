package control

import (
	"udp"
	"localqueue"
	"log"
	"time"
)

const acceptTimeoutBase=4 //seconds
const newTimeoutBase = 500 //milliseconds
const N_FLOORS=4
var globalQueue=make(map[uint]udp.Message)
var motorStop bool

//Called multiple places in this file only
//Generates key for the globalQueue based on floor and direction
func generateKey(floor uint, direction bool) uint{
	if direction{
		floor+=10
	}
	return floor
}

//Called by control.newKeyPress
//Adds new order to globalQueue and network
func addMessage(floor uint, direction bool){
	key:= generateKey(floor, direction)
	message:= udp.Message{
		LiftId:myID,
		Floor:floor,
		Direction:direction,
		Status: udp.New,
		Weight: cost(floor,direction),
		TimeRecv:time.Now()}

	if _,inQueue:=globalQueue[key];inQueue{
		return
	}
	globalQueue[key]=message
	toNetwork<-message
}

//Call by control.removeFromQueue
//Marks order as done,sends to network and deletes from globalQueue
func delMessage(floor uint, direction bool){
	key:=generateKey(floor,direction)
	if val, inQueue:=globalQueue[key];inQueue{
		val.Status=udp.Done
		toNetwork <- val
		delete(globalQueue,key)
	}
}

//Called by control.Runlift
//Handles incomming messages from network
func newMessage(message udp.Message){
	key:=generateKey(message.Floor,message.Direction)
	val,inQueue:=globalQueue[key]
	if inQueue{
		switch message.Status{
		case udp.Done:
			delete(globalQueue,key)
		case udp.Accepted:
			globalQueue[key]=message
		case udp.New:
			if val.Weight <= message.Weight{
				globalQueue[key]=message
			}
		case udp.Reassign:
			if message.ReassId!=myID{
				if val.Weight <= message.Weight{
					globalQueue[key]=message
				}
			}
		default:
			log.Println("Unknown status: ", message.Status, "Ignoring message")
		}
	}else{
		switch message.Status{
		case udp.Done:
			//Promptly ignore
		case udp.Accepted:
			if val.Status==udp.Reassign && val.ReassId==myID{
				localqueue.DeleteLocalRequest(message.Floor, message.Direction)
			}
			globalQueue[key]=message
		case udp.Reassign:
			fs:=cost(message.Floor,message.Direction)
			if fs > message.Weight{
				message.Weight=fs
				message.LiftId=myID
				globalQueue[key]=message
				toNetwork<-message
				log.Println("Reassign from lift ", message.ReassId," to ",myID)
			}else{
				globalQueue[key]=message
			}
		case udp.New:
			fs:=cost(message.Floor,message.Direction)
			if fs > message.Weight{
				message.Weight=fs
				message.LiftId=myID
				globalQueue[key]=message
				toNetwork<-message
			}else{
				globalQueue[key]=message
			}
		default:
			log.Println("Unknown status: ",message.Status, "Ignoring message")
		}
	}
}

//Called by control.RunLift
//Checks timeout on all orders in globalQueue, also check for motorstop
func checkTimeout(){
	newTimeout:=time.Duration(newTimeoutBase)
	acceptTimeout:=time.Duration(acceptTimeoutBase)
	select{
	case motorStop=<-motorStopCh:
		log.Println("MotorStop or Unstop detected. Value:",motorStop)
	default:
	}
	for key,val:=range globalQueue{
		if (val.Status==udp.New || val.Status == udp.Reassign) && !motorStop{
			timediff:= time.Now().Sub(val.TimeRecv)
			if timediff >((3*newTimeout)*time.Millisecond){
				newOrderTimeout(key,3)
			} else if timediff >((2*newTimeout)*time.Millisecond){
				newOrderTimeout(key,2)
			} else if timediff >((1* newTimeout)*time.Millisecond){
				newOrderTimeout(key,1)
			}
		} else if val.Status == udp.Accepted && val.LiftId != myID && !motorStop{
			timediff:=time.Now().Sub(val.TimeRecv)
			if timediff > ((4*acceptTimeout)*time.Second){
				acceptOrderTimeout(key,3)
			} else if timediff >((3*acceptTimeout)*time.Second){
				acceptOrderTimeout(key,2)
			} else if timediff >((2*acceptTimeout)*time.Second){
				acceptOrderTimeout(key,1)
			}
		} else if val.Status == udp.Accepted && val.LiftId==myID {
			timediff:=time.Now().Sub(val.TimeRecv)
			if motorStop {
				val.Status=udp.New
				val.Weight=0
				val.TimeRecv=time.Now()
				val.ReassId=0
				val.LiftId=0
				globalQueue[key]=val
				toNetwork<-globalQueue[key]
			} else if timediff > (acceptTimeout * time.Second){
				val.Weight=cost(val.Floor,val.Direction)
				val.TimeRecv=time.Now()
				globalQueue[key]=val
				toNetwork<-globalQueue[key]
			}
		}
	}
}

//Called by checkTimeout
func newOrderTimeout(key,critical uint){
	switch critical{
	case 3:
		takeOrder(key)
	case 2:
		if isIdle{
			takeOrder(key)
		} else if cost(globalQueue[key].Floor,globalQueue[key].Direction) > globalQueue[key].Weight{
			takeOrder(key)
		}
	case 1:
		if globalQueue[key].LiftId==myID{
			takeOrder(key)
		}
	}
}

//Called by checkTimeout
func acceptOrderTimeout(key uint, critical uint){
	switch critical{
	case 3:
		log.Println("ERROR! Reassigning orders failed. FALLBACK")
		takeOrder(key)
	case 2:
		takeOrder(key)
	case 1:
		reassignOrder(key)
	}
}

//Called by newOrderTimeout,acceptOrderTimeout and 
//Accepts order and notifies network
func takeOrder(key uint){
	log.Println("Taking order: ",globalQueue[key])
	if val,inQueue:=globalQueue[key];!inQueue{
		log.Println("Trying to accept order not in queue")
	} else{
		log.Println("Accepted order",globalQueue[key])
		val.LiftId=myID
		val.Status=udp.Accepted
		val.TimeRecv=time.Now()
		localqueue.AddLocalRequest(val.Floor,val.Direction)
		globalQueue[key]=val
		toNetwork<-globalQueue[key]
	}
}

//Called by acceptOrderTimeout
//Tries to reassign order
func reassignOrder(key uint){
	log.Println("Reassign order called on order: ",globalQueue[key])
	if val,inQueue:=globalQueue[key];!inQueue{
		log.Println("Trying to reassign order not in queue")
	} else {
		log.Println("Reassigning order",globalQueue[key])
		val.Status=udp.Reassign
		val.ReassId=myID
		val.Weight=cost(val.Floor,val.Direction)
		val.TimeRecv=time.Now()
		globalQueue[key]=val
		toNetwork<-globalQueue[key]
	}
}

//Called by multiple functions in this file
//Evaluates the cost for this elevator to execute order, higher is better
func cost(reqFloor uint, reqDir bool) int{
	statusFloor:=liftStatus.Floor
	statusDir:=liftStatus.Direction
	if isIdle{
		if reqFloor==statusFloor{
			return 6
		} else{
			return N_FLOORS+1-diff(reqFloor,statusFloor)
		}
	} else if reqDir == statusDir{
		if (statusDir && reqFloor > statusFloor) || (!statusDir && reqFloor < statusFloor){
			return N_FLOORS+1-diff(reqFloor,statusFloor)
		}
	} else {
		if (statusDir && reqFloor > statusFloor) || (!statusDir && reqFloor < statusFloor){
			return N_FLOORS+1-diff(reqFloor,statusFloor)
		}
	}
	return 1
}

//Called by cost
func diff(a uint, b uint) int{
	x:=int(a)
	y:=int(b)
	c:=x-y
	if c < 0{
		return c*-1
	} else{
		return c
	}
}
