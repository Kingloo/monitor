use std::thread::sleep;
use std::time::Duration;
use windows_bindings::Windows::Win32::WindowsAndMessaging::*;

// HWND 0xFFFF sends to all windows
// 0x0112 is WM_SYSCOMMAND
// 0xF170 is SC_MONITORPOWER
// MONITOR_ON = -1
// MONITOR_STANDBY = 1
// MONITOR_OFF = 2

fn main() -> windows::Result<()> {
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
