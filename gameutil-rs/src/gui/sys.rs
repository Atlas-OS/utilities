use std::{ffi::c_void, process::Command};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use winapi::um::processthreadsapi::OpenProcess;
use windows_dll::dll;

// https://www.aldeid.com/wiki/Process-Security-and-Access-Rights
const PROCESS_ALL_ACCESS: u32 = 0x1F0FFF;

#[dll("ntdll.dll")]
extern "system" {
    #[allow(non_snake_case)]
    fn NtSetTimerResolution(DesiredTime: u32, SetResolution: u8, PreviousTime: *mut u32) -> i32;

    #[allow(non_snake_case)]
    fn NtQueryTimerResolution(
        MaximumTime: *mut u32,
        MinimumTime: *mut u32,
        CurrentTime: *mut u32,
    ) -> i32;

    #[allow(non_snake_case)]
    fn NtSuspendProcess(ProcessHandle: *mut u32) -> i32;

    #[allow(non_snake_case)]
    fn NtResumeProcess(ProcessHandle: *mut u32) -> i32;
}

// EmptyWorkingSet
#[dll("psapi.dll")]
extern "system" {
    #[allow(non_snake_case)]
    fn EmptyWorkingSet(hProcess: *mut c_void) -> i32;
}

pub fn timerres(value: u32) {
    let mut min = 0u32;
    let mut max = 0u32;
    let mut cur = 0u32;
    unsafe {
        /*let ntstatus = */
        NtQueryTimerResolution(&mut min, &mut max, &mut cur);
        //println!("NTStatus: {:#}", ntstatus);
        //println!("Minimum: {:#}", min);
        //println!("Maximum: {:#}", max);
        //println!("Current: {:#}", cur);
        //println!("\n");

        /*let ntstatus = */
        NtSetTimerResolution(value, 1, &mut cur);
        //println!("NTStatus: {:#}", ntstatus);
        //println!("Current: {:#}", cur);
    };
}

fn suspendproc(target: &str) {
    let pid = getpid(target);
    // use ntdll to suspend process
    if let Some(pid) = pid {
        unsafe {
            let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
            /*let ntstatus = */
            NtSuspendProcess(handle as *mut u32);
            //println!("NTStatus: {:#}", ntstatus);
        };
    }
}

pub fn resumeproc(target: &str) {
    let pid = getpid(target);
    if let Some(pid) = pid {
        unsafe {
            let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
            /*let ntstatus = */
            NtResumeProcess(handle as *mut u32);
            //println!("NTStatus: {:#}", ntstatus);
        };
    }
}

pub fn taskkill(programname: &str) {
    let mut cmd = "taskkill /F /IM ".to_string();
    cmd.push_str(programname);
    Command::new("cmd").arg("/c").arg(cmd).spawn().unwrap();
}

pub fn startproc(programname: &str) {
    Command::new(programname).spawn().unwrap();
}

pub fn killdwm() {
    let p2k = [
        "explorer.exe",
        "searchapp.exe",
        "shellexperiencehost.exe",
        "searchui.exe",
        "runtimebroker.exe",
        "textinputhost.exe",
        "dllhost.exe",
        "wmiprsvse.exe",
        "dwm.exe",
    ];
    suspendproc("winlogon.exe");
    for proc in p2k.iter() {
        let pid = getpid(proc);
        if pid.is_some() {
            taskkill(proc);
        }
    }
}

// disable idle: 1
// enable idle: 0
pub fn idle(off: u8) {
    Command::new("powercfg")
        .arg("/setacvalueindex")
        .arg("scheme_current")
        .arg("sub_processor")
        .arg("5d76a2ca-e8c0-402f-a133-2158492d58ad")
        .arg(off.to_string())
        .spawn()
        .expect("failed to set power settings");
    Command::new("powercfg")
        .arg("-S")
        .arg("scheme_current")
        .spawn()
        .expect("failed to set power settings");
}

fn getpid(target: &str) -> Option<u32> {
    let mut sys = System::new();
    sys.refresh_processes();
    sys.processes()
        .iter()
        .map(|(pid, process)| {
            if process.name() == target {
                return Some(pid.to_owned().as_u32());
            }
            None
        })
        .next()
        .unwrap_or(None)
}

pub fn cleanworkingset() {
    // get list of processes
    let mut sys = System::new();
    sys.refresh_processes();
    // for every process, clear it's working set
    sys.processes().iter().for_each(|p| unsafe {
        EmptyWorkingSet(OpenProcess(PROCESS_ALL_ACCESS, 0, p.0.as_u32()));
    })
}

// TODO: args for more hotkeys and callbacks
pub fn hotkey() -> livesplit_hotkey::Hook {
    let hotkey = livesplit_hotkey::Hook::new().expect("failed to make new hotkey!");
    hotkey
        .register(livesplit_hotkey::KeyCode::F4, move || {
            cleanworkingset();
        })
        .expect("failed to register hotkey!");
    hotkey
}
