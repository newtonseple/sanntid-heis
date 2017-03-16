package main

import (
	"flag"
	"log"
	"os"
	"os/exec"
	"os/signal"
	"syscall"
	"time"
	"control"
)

var backupflag = flag.Bool("backup", false, "Start as backup process")
var quitCh = make(chan bool)

func main() {
	flag.Parse()
	if *backupflag {
		backup() //Blockingcall for the backup
	}
	log.Println("Starting elevator. Send SIGQUIT to shutown (CTRL+\\)")
	cmd := spawnBackup()
	go signaler(cmd)
	control.RunLift(quitCh)
	log.Println("Lift Shutdown")
}

//Called by main
func spawnBackup() *exec.Cmd {
	cmd := exec.Command(os.Args[0], "-backup")
	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout
	err := cmd.Start()
	if err != nil {
		log.Fatal(err)
	}
	return cmd
}

//Call by main
//Signals the backup every 200 millisecond
func signaler(cmd *exec.Cmd) {
	ticker := time.NewTicker(200 * time.Millisecond).C
	sigint := make(chan os.Signal, 1)
	signal.Notify(sigint, syscall.SIGINT, syscall.SIGQUIT)
	for {
		select {
		case <-ticker:
			err := cmd.Process.Signal(syscall.SIGUSR1)
			if err != nil {
				log.Println("tick error: ", err)
			}
		case sig := <-sigint:
			log.Println("Main caught", sig, ". Trying to clean up")
			log.Println("Main exit in 200 millisecond")
			close(quitCh)
			time.Sleep(200 * time.Millisecond)
			os.Exit(1)
		}
	}
}

//Called by main
//If main dies, we get missed signals -> kill the parent and return
func backup() {
	missed_signal := 0
	sigint := make(chan os.Signal, 1)
	sigquit := make(chan os.Signal, 1)
	sigusr := make(chan os.Signal, 1)
	signal.Notify(sigint, syscall.SIGINT)
	signal.Notify(sigquit, syscall.SIGQUIT)
	signal.Notify(sigusr, syscall.SIGUSR1)
	ppid := os.Getppid()
	log.Println("Backup started by pid: ", ppid)
	for {
		select {
		case <-sigusr:
			missed_signal = 0
			time.Sleep(200 * time.Millisecond)
		case <-sigint:
			log.Println("Backup caught SIGINT, ignore")
		case <-sigquit:
			log.Println("Received SIGQUIT. Shutting down backup")
			os.Exit(0)
		default:
			if missed_signal == 3 {
				killParent(ppid)
				return
			}
			missed_signal += 1
			time.Sleep(200 * time.Millisecond)
		}
	}
}

//Called by backup
//Kills the parent of the backup
func killParent(ppid int) {
	if ppid != os.Getppid() {
		log.Println("Main dead. Backup now belongs to pid:", os.Getppid())
	} else {
		log.Println("Missing signals, shutting down main for restart")
		syscall.Kill(ppid, syscall.SIGINT)
		<-time.After(300 * time.Millisecond)
		if ppid == os.Getppid() {
			syscall.Kill(ppid, syscall.SIGKILL)
		}

	}
	log.Println("Backup going down")
	return
}
