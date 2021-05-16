fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{LRESULT, PWSTR},
        Windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcW, RegisterClassExW, HWND, LPARAM, WM_LBUTTONDOWN,
            WM_RBUTTONDOWN, WNDCLASSEXW, WPARAM,
        },
    );
}
