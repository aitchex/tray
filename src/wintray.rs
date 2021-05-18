use std::ptr;

use bindings::Windows::Win32::{
    System::SystemServices::{self, HINSTANCE, LRESULT, PSTR},
    UI::WindowsAndMessaging::{
        self, HWND, LPARAM, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WNDCLASSA, WPARAM,
    },
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
            return WindowsAndMessaging::DefWindowProcA(hwnd, msg, wparam, lparam);
        },
    }

    LRESULT(0)
}

pub struct WinTray {}

impl WinTray {
    fn register_class(class_name: &str, instance: HINSTANCE) -> u16 {
        let wc = WNDCLASSA {
            lpszClassName: PSTR(class_name.as_bytes().as_ptr() as _),
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            ..Default::default()
        };
        let result = unsafe { WindowsAndMessaging::RegisterClassA(&wc) };

        result
    }

    fn create_window(class_name: &str, instance: HINSTANCE) -> HWND {
        let handle = unsafe {
            WindowsAndMessaging::CreateWindowExA(
                Default::default(),
                class_name,
                PSTR::default(),
                Default::default(),
                0,
                0,
                0,
                0,
                None,
                None,
                instance,
                ptr::null_mut(),
            )
        };

        handle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_class() {
        let class_name = "Register";
        let instance = unsafe { SystemServices::GetModuleHandleA(None) };
        let result = WinTray::register_class(class_name, instance);
        assert_ne!(result, 0);
    }

    #[test]
    fn test_create_window() {
        let class_name = "Window";
        let instance = unsafe { SystemServices::GetModuleHandleA(None) };
        WinTray::register_class(class_name, instance);
        let handle = WinTray::create_window(class_name, instance);
        assert_ne!(handle.0, 0);
    }
}
