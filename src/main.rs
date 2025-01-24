use std::thread::sleep;
use std::time::Duration;
use windows::core::Result;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, HWND_BROADCAST, SC_MONITORPOWER, WM_SYSCOMMAND};

// HWND_BROADCAST sends to all windows (const of 0xFFFF)
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
			HWND_BROADCAST,
			WM_SYSCOMMAND,
			Some(WPARAM(SC_MONITORPOWER as usize)),
			Some(LPARAM(2)),
		);
	};
	Ok(())
}
