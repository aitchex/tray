fn main() {
    windows::build!(
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::System::SystemServices::CHAR,
        Windows::Win32::Foundation::{HINSTANCE, LRESULT, PSTR, PWSTR, HWND, LPARAM, WPARAM},
        Windows::Win32::UI::Controls::IMAGE_FLAGS,
        Windows::Win32::UI::Shell::{
             NOTIFYICONDATAA, Shell_NotifyIconA,
        },
        Windows::Win32::UI::WindowsAndMessaging::{
            DefWindowProcA, RegisterClassA, WM_LBUTTONUP, WM_RBUTTONUP,
            WNDCLASSA, WINDOW_STYLE, CreateWindowExA, MSG, GetMessageA, TranslateMessage,
            DispatchMessageA, WM_APP, WM_QUIT, LoadImageW,  DestroyIcon, HICON,
        },
    );
}
