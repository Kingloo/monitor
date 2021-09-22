use std::thread::sleep;
use std::time::Duration;
use windows_bindings::Windows::Win32::Foundation::{LPARAM, WPARAM, HWND};
use windows_bindings::Windows::Win32::UI::WindowsAndMessaging::{
	SendMessageW, SC_MONITORPOWER, WM_SYSCOMMAND,
};

// HWND 0xFFFF sends to all windows
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
