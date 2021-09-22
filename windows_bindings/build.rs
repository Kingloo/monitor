fn main() {
	windows::build!(
		Windows::Win32::UI::WindowsAndMessaging::{SendMessageW, SC_MONITORPOWER, WM_SYSCOMMAND},
		Windows::Win32::Foundation::{HWND}
	);
}
