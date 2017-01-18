from threading import Thread, Lock

i = 0
lock = Lock()

def thread_1_func():
    global i
    for n in range(1000000):
            lock.acquire()
            i += 1
            lock.release()

def thread_2_func():
    global i
    for n in range(1000000):
        lock.acquire()
        i -= 1
        lock.release()

def main():
    threads = [Thread(target = thread_1_func, args = ()),
               Thread(target = thread_2_func, args = ())]
    for thread in threads: thread.start()
    for thread in threads: thread.join()
    print(i)

main()
