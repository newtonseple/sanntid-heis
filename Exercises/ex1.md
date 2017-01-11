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
* What is concurrency? What is parallelism? What's the difference?
	- Parallellism: Threads running at the same instant
	- Concurrency: Threads running in the same time interval
* Why have machines become increasingly multicore in the past decade?

* What kinds of problems motivates the need for concurrent execution? (Or phrased differently: What problems do concurrency help in solving?)

* Does creating concurrent programs make the programmer's life easier? Harder? Maybe both? (Come back to this after you have worked on part 4 
* of this exercise)

* What are the differences between processes, threads, green threads, and coroutines?

* Which one of these do pthread_create() (C/POSIX), threading.Thread() (Python), go (Go) create?

* How does pythons Global Interpreter Lock (GIL) influence the way a python Thread behaves?

* With this in mind: What is the workaround for the GIL (Hint: it's another module)?

* What does func GOMAXPROCS(n int) int change?