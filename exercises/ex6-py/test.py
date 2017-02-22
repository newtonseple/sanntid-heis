from sys import executable
from subprocess import Popen, CREATE_NEW_CONSOLE

from time import time, sleep
from os import mkdir, listdir, rename

def get_state():
    try:
        return tuple([float(i) for i in [s.split('_')[1:3] for s in listdir('.') if "STATE" in s][0]])
    except:
        return (0.0,0.0)

def set_state(n):
    state = get_state()    
    if state != (0,0):
        rename("STATE_"+str(state[0])     +"_"+str(state[1]),
               "STATE_"+str(float(time()))+"_"+str(float(n)))
    else:
        mkdir("STATE_"+str(float(time()))+"_"+str(float(n)))

timeout = 2
period = 1

if not get_state()[0] + timeout > time():
    set_state(0)
else:
    while(get_state()[0] + timeout > time()):
        sleep(period)
    set_state(get_state()[1])

Popen(["start",executable, __file__], shell=True)

try:
    while(1):
        print int(get_state()[1]+1)
        set_state(get_state()[1]+1)
        sleep(period)
except:
    input("exit?")