extern crate winapi;

use std::time::Duration;
use std::thread::sleep;
use winapi::shared::windef::HWND;
use winapi::um::winuser::SendMessageW;

fn set_all_monitors_to_standby() {
    // 0xffff sends the message to all top-level windows
    let all_windows: HWND = 0xFFFF as HWND;
    let wm_syscommand: u32 = 0x0112;
    let sc_monitorpower: usize = 0xF170;
    // MONITOR_ON = -1
    // MONITOR_STANDBY = 1
    // MONITOR_OFF = 2
    let monitor_off: isize = 2;
    
    unsafe {
        SendMessageW(all_windows, wm_syscommand, sc_monitorpower, monitor_off)
    };
}

fn main() {
    sleep(Duration::from_secs(1));
    set_all_monitors_to_standby();
}