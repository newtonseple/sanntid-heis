from threading import Thread
i = 0
def thread_1_func():
    global i
    for n in range(1000000): i += 1
def thread_2_func():
    global i
    for n in range(1000000): i -= 1
def main():
    threads = [Thread(target = thread_1_func, args = ()),
               Thread(target = thread_2_func, args = ())]
    for thread in threads: thread.start()
    for thread in threads: thread.join()
    print(i)
main()