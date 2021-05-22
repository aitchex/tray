fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{GetModuleHandleA, LRESULT, PSTR, CHAR},
        Windows::Win32::UI::Shell::{
            NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAA, Shell_NotifyIconA,
        },
        Windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcA, HWND, LPARAM, RegisterClassA, WM_LBUTTONDOWN, WM_RBUTTONDOWN,
            WNDCLASSA, WPARAM, WINDOW_STYLE, CreateWindowExA,
        },
        Windows::Win32::UI::MenusAndResources::HICON,
    );
}
