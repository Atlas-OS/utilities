// coauthored by @spddl, thank you!
package main

import (
	"syscall"

	"golang.org/x/sys/windows"
)

// Breaks when calling from main.go, just replaced with 0x1F0FFF
// const PROCESS_ALL_ACCESS = 0x1F0FF

var (
	// Library
	ntdllDLL = windows.NewLazySystemDLL("ntdll.dll")

	// Functions
	procNtSuspendProcess = ntdllDLL.NewProc("NtSuspendProcess")
	procNtResumeProcess  = ntdllDLL.NewProc("NtResumeProcess")
	// procNtQueryTimerResolution = ntdllDLL.NewProc("NtQueryTimerResolution")
	procNtSetTimerResolution = ntdllDLL.NewProc("NtSetTimerResolution")
)

func SuspendProc(pid uint32) {
	handle, err := windows.OpenProcess(0x1F0FFF, false, pid)
	if err != nil {
		panic(err)
	}
	procNtSuspendProcess.Call(uintptr(handle))
}

func NtResumeProcess(pid uint32) {
	handle, err := windows.OpenProcess(0x1F0FFF, false, pid)
	if err != nil {
		panic(err)
	}
	procNtResumeProcess.Call(uintptr(handle))
}

/*
// https://www.pinvoke.net/default.aspx/ntdll.NtQueryTimerResolution
// http://undocumented.ntinternals.net/index.html?page=UserMode%2FUndocumented%20Functions%2FTime%2FNtQueryTimerResolution.html
func NtQueryTimerResolution(maximumResolution, minimumResolution, currentResolution *uint64) bool {
	ret, _, _ := syscall.Syscall(procNtQueryTimerResolution.Addr(), 3,
		uintptr(unsafe.Pointer(maximumResolution)),
		uintptr(unsafe.Pointer(minimumResolution)),
		uintptr(unsafe.Pointer(&currentResolution)),
	)
	return ret != 0
}
*/

func NtSetTimerRes(res int16) (currentResolution uint64) {
	syscall.Syscall(procNtSetTimerResolution.Addr(), 3,
		uintptr(res),
		uintptr(1),
		// Believe this isn't required, and adds extra module.
		// uintptr(unsafe.Pointer(&currentResolution)),
		0,
	)
	return
}
