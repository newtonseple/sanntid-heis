// Go 1.2
// go run helloworld_go.go

package main

import (
    . "fmt"
    "runtime"
    "time"
)


func thread_1_func(i_glob chan int) {
    for n := 0 ; n < 1000000 ; n++{
        i := <- i_glob
        i++
        i_glob <- i
    }
}
func thread_2_func(i_glob chan int) {
    var i int = 0
    for n := 0 ; n < 1000000 ; n++{
        i := <- i_glob
        i--
        i_glob <- i
    }
}


func main() {
    runtime.GOMAXPROCS(runtime.NumCPU())    // I guess this is a hint to what GOMAXPROCS does...
                                            // Try doing the exercise both with and without it!
    i_glob := make(chan int)

    go thread_1_func(i_glob)                      // This spawns someGoroutine() as a goroutine
    go thread_2_func(i_glob) 

    // We have no way to wait for the completion of a goroutine (without additional syncronization of some sort)
    // We'll come back to using channels in Exercise 2. For now: Sleep.
    time.Sleep(100*time.Millisecond)
    Println(i)
}