fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::LRESULT,
        Windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcW, HWND, LPARAM, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WPARAM,
        },
    );
}
