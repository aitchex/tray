fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{GetModuleHandleA, LRESULT, PSTR},
        Windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcA, HWND, LPARAM, RegisterClassA, WM_LBUTTONDOWN, WM_RBUTTONDOWN,
            WNDCLASSA, WPARAM, WINDOW_STYLE, CreateWindowExA,
        },
    );
}
