use bindings::Windows::Win32::{
    System::SystemServices::{LRESULT, PWSTR},
    UI::WindowsAndMessaging::{
        self, HWND, LPARAM, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WNDCLASSEXW, WPARAM,
    },
};
use windows::HSTRING;

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

pub struct WinTray {}

impl WinTray {
    fn register() -> u16 {
        let class_name = HSTRING::from("Tray Class").as_wide().to_owned();
        let wcex = WNDCLASSEXW {
            lpfnWndProc: Some(window_proc),
            lpszClassName: PWSTR(class_name.as_ptr() as _),
            ..Default::default()
        };

        let result = unsafe { WindowsAndMessaging::RegisterClassExW(&wcex) };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let result = WinTray::register();
        assert_eq!(result, 0);
    }
}
