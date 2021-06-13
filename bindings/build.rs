fn main() {
    windows::build!(
        Windows::Win32::System::Diagnostics::Debug::GetLastError,
        Windows::Win32::System::SystemServices::{GetModuleHandleA, CHAR, HINSTANCE, LRESULT, PSTR, PWSTR},
        Windows::Win32::UI::Controls::LR_LOADFROMFILE,
        Windows::Win32::UI::MenusAndResources::HICON,
        Windows::Win32::UI::Shell::{
            NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_MODIFY, NOTIFYICONDATAA, Shell_NotifyIconA,
        },
        Windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcA, HWND, LPARAM, RegisterClassA, WM_LBUTTONUP, WM_RBUTTONUP,
            WNDCLASSA, WPARAM, WINDOW_STYLE, CreateWindowExA, MSG, GetMessageA, TranslateMessage,
            DispatchMessageA, WM_APP, WM_QUIT, LoadImageW, IMAGE_ICON, DestroyIcon,
        },
    );
}
