fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{GetModuleHandleA, LRESULT, PSTR, CHAR},
        Windows::Win32::UI::Shell::{
            NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAA, Shell_NotifyIconA,
        },
        Windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcA, HWND, LPARAM, RegisterClassA, WM_LBUTTONUP, WM_RBUTTONUP,
            WNDCLASSA, WPARAM, WINDOW_STYLE, CreateWindowExA, MSG, GetMessageA, TranslateMessage,
            DispatchMessageA, WM_APP, WM_QUIT,
        },
        Windows::Win32::UI::MenusAndResources::HICON,
    );
}
