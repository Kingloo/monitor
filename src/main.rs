use std::thread::sleep;
use std::time::Duration;
use windows::core::Result;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, SC_MONITORPOWER, WM_SYSCOMMAND};

// HWND 0xFFFF sends to all windows
// WPARAM is the command to send, in this case SC_MONITORPOWER
// LPARAM is the value to pass, one of the following
//		MONITOR_ON = -1
//		MONITOR_STANDBY = 1
//		MONITOR_OFF = 2
// https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syscommand

fn main() -> Result<()> {
	sleep(Duration::from_secs(1));
	unsafe {
		SendMessageW(
			HWND(0xFFFF),
			WM_SYSCOMMAND,
			WPARAM(SC_MONITORPOWER as usize),
			LPARAM(2),
		);
	};
	Ok(())
}
