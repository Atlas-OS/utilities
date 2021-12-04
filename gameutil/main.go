package main

import (
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"strconv"
	"syscall"

	"golang.org/x/sys/windows"
)

func killProcesses(pids []string) {
	for _, pid := range pids {
		// convert string to uint64
		pidUint, err := strconv.ParseUint(pid, 10, 32)
		if err != nil {
			fmt.Println(err)
			continue
		}
		// https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess
		// open the process and get the handle
		pHndl, err := windows.OpenProcess(windows.PROCESS_TERMINATE, false, uint32(pidUint))
		defer windows.CloseHandle(pHndl)
		if err != nil {
			fmt.Println("windows.OpenProcess", err)
			continue
		}
		if pHndl == 0 {
			fmt.Println("no handle")
			continue
		}
		// close the process
		err = windows.TerminateProcess(pHndl, 0)
		if err != nil {
			fmt.Printf("TerminateProcess(pid: %s/handle: %v): %v\n", pid, pHndl, err)
		}
	}
}

func main() {
	// get config...
	c := GetConfig()
	var wp Processes
	err := wp.getProcesses()
	if err != nil {
		panic(err)
	}
	if c.KILL_EXPLORER {
		fmt.Println("Killing explorer..")
		pids := wp.findProcessIDByNames([]string{"explorer.exe"})
		killProcesses(pids)
	}
	if c.TIMERRES != 0 {
		NtSetTimerRes(c.TIMERRES)
	}
	if c.DISABLE_IDLE {
		err := exec.Command("powercfg", "-setacvalueindex", "scheme_current", "sub_processor", "5d76a2ca-e8c0-402f-a133-2158492d58ad", "1").Start()
		if err != nil {
			fmt.Printf("Could not disable idle: %v\n", err)
		}
	}
	if c.KILL_DWM {
		// Kill explorer and other processes
		p2k := []string{"explorer.exe", "searchapp.exe", "shellexperiencehost.exe", "searchui.exe", "runtimebroker.exe", "textinputhost.exe", "dllhost.exe", "wmiprvse.exe"}
		pids := wp.findProcessIDByNames(p2k)
		fmt.Println("p2k", pids)
		killProcesses(pids)

		// Suspend winlogon
		fmt.Println("Suspending winlogon..")
		pids = wp.findProcessIDByNames([]string{"winlogon.exe"})
		fmt.Println("winlogon", pids)
		pidUint, _ := strconv.ParseUint(pids[0], 10, 32)
		SuspendProc(uint32(pidUint))
		// Kill DWM
		pids = wp.findProcessIDByNames([]string{"dwm.exe"})
		fmt.Println("dwm", pids)
		killProcesses(pids)
	}

	fmt.Println("Done! Alt-Tab to your game. Once you are finished, press any key to restore default functionality.")
	var input string
	fmt.Scanln(&input)

	// restore explorer, resume winlogon, dwm etc.
	// timer resolution will be returned back to previous value after process is closed
	if c.DISABLE_IDLE {
		err := exec.Command("powercfg", "-setacvalueindex", "scheme_current", "sub_processor", "5d76a2ca-e8c0-402f-a133-2158492d58ad", "0").Start()
		if err != nil {
			fmt.Printf("Could not enable idle: %v\n", err)
		}
	}
	if c.KILL_DWM {
		// Resume winlogon
		fmt.Println("Resuming winlogon..")
		pids := wp.findProcessIDByNames([]string{"winlogon.exe"})
		for _, pid := range pids {
			pidUint, err := strconv.ParseUint(pid, 10, 32)
			if err != nil {
				fmt.Println(err)
				continue
			}
			NtResumeProcess(uint32(pidUint))
		}
		pids = wp.findProcessIDByNames([]string{"explorer.exe"})
		killProcesses(pids)
		ch := make(chan os.Signal)
		signal.Notify(ch, os.Interrupt, syscall.SIGTERM)
		go func() {
			<-ch
			err := exec.Command("explorer.exe").Start()
			if err != nil {
				fmt.Printf("Explorer restart failed with %s\n", err)
			}
			os.Exit(0)
		}()
	}
	if c.KILL_EXPLORER {
		fmt.Println("Restarting explorer..")
		err := exec.Command("explorer.exe").Start()
		if err != nil {
			fmt.Printf("Explorer restart failed with %s\n", err)
		}
	}
	os.Exit(0)
}
