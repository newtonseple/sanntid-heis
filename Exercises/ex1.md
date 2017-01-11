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
	- Size and cost reduced, but power input and heat output remains, and increases with frequency. Solution: More cores
* What kinds of problems motivates the need for concurrent execution? (Or phrased differently: What problems do concurrency help in solving?)
	- Multiple tasks that need to respond or run in a timely manner.
* Does creating concurrent programs make the programmer's life easier? Harder? Maybe both? (Come back to this after you have worked on part 4 of this exercise)
	- Easier: Modularizability
	- Harder: Might need to deal with shared memory
* What are the differences between processes, threads, green threads, and coroutines?
	- Process: OS-managed, own memory
	- Thread: OS-managed, shared memory
	- Green thread: Threads, not OS-managed
	- Coroutine: Threads, not preemptive, not OS-managed. 
* Which one of these do X create?
* X = pthread_create() (C/POSIX), 
	- Thread
* X = threading.Thread() (Python), 
	- Thread
* X = go (Go)
	- Coroutines-ish ???
* How does pythons Global Interpreter Lock (GIL) influence the way a python Thread behaves?
	- Only one thread running python code simultaneously
* With this in mind: What is the workaround for the GIL (Hint: it's another module)?
	- subprocess/multiprocessing
* What does func GOMAXPROCS(n int) int change?
	- The maximum number of CPUs that can be executing simultaneously.