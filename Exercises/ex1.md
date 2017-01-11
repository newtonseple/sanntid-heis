TTK4145 Exercise 1
==================
Problem 1
---------
* What if the software controlling one of the elvators suddeny crashes?
	- The rest already knows its tasks, and takes over if it does not recover.
* What if it doesn't crash, but hangs?
	- Same thing. I-am-alive system times out.
* What if a message between machines is lost?
	- Handled by TCP -> Retry/Fail
* What if the network cable is suddenly disconnected? Then re-connected?
	- Same as crash/hang.
* What if a user of the system is being a troll?
	- U mad?
* What if the elevator car never arrives at its destination?
	- If program error: I-am-alive system times out.
	- If physical: Self-check -> Retry/Fail

Problem 3
---------
* Parallellism: Threads running at the same instant
* Concurrency: Threads running in the same time interval

