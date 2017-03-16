# Exercise 9 : Scheduling


## Properties

**Task 1:**
 1. Why do we assign priorities to tasks?
 	- Some tasks are more time sensitive than others. Notably, UI must not make a user wait for it, and drivers must satisfy specifications.
 2. What features must a scheduler have for it to be usable for real-time systems?
 	- The scheduler need to be predictable, so that it is possible to analyze the system. We need to analyze the system sp that we can be sure that we keep our timing-constraints. 


## Inversion and inheritance

*Task set 1:*

| Task | Priority   | Execution sequence | Release time |
|------|------------|--------------------|--------------|
| a    | 3          | `E Q V E`          | 4            |
| b    | 2          | `E V V E E E`      | 2            |
| c    | 1 (lowest) | `E Q Q Q E`        | 0            |

*Where:*
 - `E` : Executing
 - `Q` : Executing with resource Q locked
 - `V` : Executing with resource V locked

**Task 2:** Draw Gantt charts to show how task set 1 executes:
 1. Without priority inheritance

| Task/Time | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
|-----------|---|---|---|---|---|---|---|---|---|---|----|----|----|----|----|
| a         | - | - | - | - | E | - | - | - | - | - |  - |  Q |  V |  E |  - |
| b         | - | - | E | V | - | V | E | E | E | - |  - |  - |  - |  - |  - |
| c         | E | Q | - | - | - | - | - | - | - | Q |  Q |  - |  - |  - |  E |

 2. With priority inheritance

| Task/Time | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
|-----------|---|---|---|---|---|---|---|---|---|---|----|----|----|----|----|
| a         | - | - | - | - | E | - | - | Q | - | V |  E |  - |  - |  - |  - |
| b         | - | - | E | V | - | - | - | - | V | - |  - |  E |  E |  E |  - |
| c         | E | Q | - | - | - | Q | Q | - | - | - |  - |  - |  - |  - |  E |

**Task 3:** Explain:
 1. What is priority inversion? What is unbounded priority inversion?
 	- Priority inversion is when a low priorty task blocks a high priority task due to a locked resource.
	- Unbounded priority inversion occurs if the low priority task is preemptivly interupted by a medium priority task. This leads to the high priority task waiting for the medium priority task, effectively giving the medium priority task the highest priority.
 3. Does priority inheritance avoid deadlocks?
 	- No, the high priority task can lock a resource needed by a low priority task that has already locked a resource then needed by the high priority task.


## Utilization and response time

*Task set 2:*

| Task | Period (T) | Exec. Time (C) |
|------|------------|----------------|
| a    | 50         | 15             |
| b    | 30         | 10             |
| c    | 20         | 5              |

**Task 4:**
 1. There are a number of assumptions/conditions that must be true for the utilization and response time tests to be usable (The "simple task model"). What are these assumptions? Comment on how realistic they are.
 	- Fixed set of tasks (This requires no tasks that appear, and that the workload is known)
	- All tasks are periodic, with known periods (Often but not always realistic)
	- The tasks are independent (Often possible)
	- No system overhead (The overhead can often be ignored, but always exists)
	- All tasks have deadlines equal to their period (Acceptable in most cases)
	- All tasks have a fixed worst-case execution time (Often difficult to get a tight bound)
	- No task contains any internal suspension point (Difficult in UI)
	- All tasks execute on a single processor(thread) (Realistic for microcontrollers)
 2. Perform the utilization test for task set 2. Is the task set schedulable?
 	- U = 15/50+10/30+5/20 = 0.8833
	- 3(2^(1/3)-1) = 0.7798
	- This means that the task set may or may not schedulable
 3. Perform response-time analysis for task set 2. Is the task set schedulable? If you got different results than in 2), explain why.
 	- Assuming rate-monotonic-priority assignment:
 	- Task c
	 	+ w0 = 5 <= 20 -> OK
	- Task b
		+ w0 = 10 <= 30
		+ w1 = 15 <= 30
		+ w2 = 15 <= 30 -> OK
	- Task a
		+ w0 = 15 <= 50
		+ w1 = 30 <= 50
		+ w2 = 35 <= 50
		+ w3 = 45 <= 50
		+ w4 = 50 <= 50
		+ w5 = 50 <= 50 -> OK
	- As all the tasks are OK, the task set is schedulable.
	- The utilization test is sufficient but not necessary, however the response time analysis is both necessary and sufficient.
 4. (Optional) Draw a Gantt chart to show how task set 2 executes using rate monotonic priority assignment, and verify that your conclusions are correct.
| Task/Time| 0 | 5 | 10 | 15 | 20 | 25 | 30 | 35 | 40 | 45 | 50 | 55 | 60 | 65 | 70 | 75 | 80 | 85 | 90 | 95 | 100 | 105 | 110 | 115 | 120 | 125 | 130 | 135 | 140 | 145 |
|----------|---|---|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| a        | - | - | -  | a  | -  | a  | -  | -  | -  | a  | a  | a  | -  | -  | -  | a  | -  | -  | -  | -  |  -  |  a  |  a  |  a  |  -  |  -  |  -  |  -  |  -  |  -  |
| b        | - | b | b  | -  | -  | -  | b  | b  | -  | -  | -  | -  | -  | b  | b  | -  | -  | -  | b  | b  |  -  |  -  | -   |  -  |  -  |  b  |  b  |  -  |  -  |  -  |
| c        | c | - | -  | -  | c  | -  | -  | -  | c  | -  | -  | -  | c  | -  | -  | -  | c  | -  | -  | -  |  c  |  -  | -   |  -  |  c  |  -  |  -  |  -  |  c  |  -  |

### Formulas

Utilization:  
![U = \sum_{i=1}^{n} \frac{C_i}{T_i} \leq n(2^{\frac{1}{n}}-1)](eqn-utilization.png)

Response-time:  
![w_{i}^{n+1} = C_i + \sum_{j \in hp(i)} \bigg \lceil {\frac{w_i^n}{T_j}} \bigg \rceil C_j](eqn-responsetime.png)