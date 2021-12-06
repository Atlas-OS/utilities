package main

import (
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"strconv"
	"syscall"

	"golang.org/x/sys/windows"
	// walk
	"github.com/lxn/walk"

	//lint:ignore ST1001 standard behavior lxn/walk
	. "github.com/lxn/walk/declarative"
)

type MyMainWindow struct {
	*walk.MainWindow
	*walk.NumberEdit
}

var (
	TIMERRES      int16
	KILL_DWM      = false
	KILL_EXPLORER = false
	DISABLE_IDLE  = false
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

// use taskkill for explorer, killProcesses() makes explorer restart itself
func killExplorer() {
	exec.Command("taskkill", "/f", "/im", "explorer.exe").Run()
}

func main() {
	var label *walk.Label
	mw := new(MyMainWindow)
	if err := (MainWindow{
		AssignTo: &mw.MainWindow,
		Title:    "GameUtil",
		Size:     Size{Width: 200, Height: 110},
		Layout:   VBox{},
		Children: []Widget{
			Label{
				AssignTo: &label,
				Name:     "Label",
				Text:     "Launch your game before clicking Start",
			},
			Composite{
				Layout: Grid{Columns: 2},
				Children: []Widget{
					Label{
						Text:        "Timer Resolution:",
						ToolTipText: "In 100ns units. 0 to disable, 5000 for 0.5ms timer.",
					},
					NumberEdit{
						AssignTo: &mw.NumberEdit,
						Value:    5000,
						Decimals: 0,
						MinValue: 0,
						MaxValue: 10000,
						OnValueChanged: func() {
							TIMERRES = int16(mw.NumberEdit.Value())
						},
					},
					RadioButtonGroup{
						Buttons: []RadioButton{
							{
								Text: "Kill DWM",
								OnClicked: func() {
									KILL_DWM = true
									KILL_EXPLORER = false
								},
							},
							{
								Text: "Kill Explorer",
								OnClicked: func() {
									KILL_EXPLORER = true
									KILL_DWM = false
								},
							},
						},
					},
					Label{
						Text: "Disable Idle",
					},
					CheckBox{
						OnClicked: func() {
							DISABLE_IDLE = !DISABLE_IDLE
						},
					},
					PushButton{
						Text: "Start",
						OnClicked: func() {
							start()
						},
					},
					PushButton{
						Text: "Restore",
						OnClicked: func() {
							restore()
						},
					},
				},
			},
		},
	}).Create(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	mw.Show()
	mw.Run()
}

func start() {
	var wp Processes
	err := wp.getProcesses()
	if err != nil {
		fmt.Println(err)
	}
	if KILL_EXPLORER {
		fmt.Println("Killing explorer..")
		killExplorer()
	}
	if TIMERRES != 0 {
		NtSetTimerRes(TIMERRES)
	}
	if DISABLE_IDLE {
		exec.Command("powercfg", "-setacvalueindex", "scheme_current", "sub_processor", "5d76a2ca-e8c0-402f-a133-2158492d58ad", "1").Start()
		exec.Command("powercfg", "-setactive", "scheme_current").Start()
	}
	if KILL_DWM {
		// Kill explorer and other processes
		p2k := []string{"explorer.exe", "searchapp.exe", "shellexperiencehost.exe", "searchui.exe", "runtimebroker.exe", "textinputhost.exe", "dllhost.exe", "wmiprvse.exe"}
		pids := wp.findProcessIDByNames(p2k)
		fmt.Println("p2k", pids)
		killProcesses(pids)
		// Suspend winlogon
		pids = wp.findProcessIDByNames([]string{"winlogon.exe"})
		fmt.Println("winlogon", pids)
		pidUint, _ := strconv.ParseUint(pids[0], 10, 32)
		SuspendProc(uint32(pidUint))
		// Kill DWM
		pids = wp.findProcessIDByNames([]string{"dwm.exe"})
		fmt.Println("dwm", pids)
		killProcesses(pids)
	}
}

func restore() {
	var wp Processes
	err := wp.getProcesses()
	if err != nil {
		fmt.Println(err)
	}
	// restore explorer, resume winlogon, dwm etc.
	// timer resolution will be returned back to previous value after process is closed
	if DISABLE_IDLE {
		exec.Command("powercfg", "-setacvalueindex", "scheme_current", "sub_processor", "5d76a2ca-e8c0-402f-a133-2158492d58ad", "0").Start()
		exec.Command("powercfg", "-setactive", "scheme_current").Start()
	}
	if KILL_DWM {
		// Resume winlogon
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
	if KILL_EXPLORER {
		pids := wp.findProcessIDByNames([]string{"explorer.exe"})
		killProcesses(pids)
		ch := make(chan os.Signal)
		signal.Notify(ch, os.Interrupt, syscall.SIGTERM)
		go func() {
			<-ch
			err := exec.Command("explorer.exe").Start()
			if err != nil {
				fmt.Printf("Explorer restart failed with %s\n", err)
			}
		}()
		exec.Command("explorer.exe").Start()
	}
	os.Exit(0)
}
