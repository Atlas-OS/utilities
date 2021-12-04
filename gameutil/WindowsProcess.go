// authored by @spddl, thank you!
package main

import (
	"strconv"
	"strings"
	"syscall"
	"unsafe"

	"golang.org/x/sys/windows"
)

const TH32CS_SNAPPROCESS = 0x00000002

type Processes struct {
	Processes []Process
}

type Process struct {
	ProcessID       int
	ParentProcessID int
	Exe             string
}

func (wp *Processes) getProcesses() error {
	handle, err := windows.CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
	if err != nil {
		return err
	}
	defer windows.CloseHandle(handle)

	var entry windows.ProcessEntry32
	entry.Size = uint32(unsafe.Sizeof(entry))

	// get the first process
	err = windows.Process32First(handle, &entry)
	if err != nil {
		return err
	}

	for {
		wp.Processes = append(wp.Processes, newWindowsProcess(&entry))

		err = windows.Process32Next(handle, &entry)
		if err != nil {
			// windows sends ERROR_NO_MORE_FILES on last process
			if err == syscall.ERROR_NO_MORE_FILES {
				return nil
			}
			return err
		}
	}
}

func (wp Processes) findProcessIDByNames(names []string) []string {
	var result []string
	for _, p := range wp.Processes {
		for _, name := range names {
			if strings.EqualFold(p.Exe, name) {
				result = append(result, strconv.Itoa(p.ProcessID))
			}
		}

	}
	return result
}
func newWindowsProcess(e *windows.ProcessEntry32) Process {
	// Find when the string ends for decoding
	end := 0
	for {
		if e.ExeFile[end] == 0 {
			break
		}
		end++
	}
	return Process{
		ProcessID:       int(e.ProcessID),
		ParentProcessID: int(e.ParentProcessID),
		Exe:             syscall.UTF16ToString(e.ExeFile[:end]),
	}
}
