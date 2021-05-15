use bindings::Windows::Win32::{
    System::SystemServices::LRESULT,
    UI::WindowsAndMessaging::{self, HWND, LPARAM, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WPARAM},
};

const ICON_ID: u32 = 1;

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        ICON_ID => match lparam.0 as u32 {
            WM_LBUTTONDOWN => println!("Left Click!"),
            WM_RBUTTONDOWN => println!("Right Click!"),
            _ => (),
        },
        _ => unsafe {
            return WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam);
        },
    }

    LRESULT(0)
}
