use bindings::Windows::Win32::{
    System::SystemServices::{LRESULT, PSTR},
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
    fn register_class() -> u16 {
        let wc = WNDCLASSA {
            lpszClassName: PSTR(b"window\0".as_ptr() as _),
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        };
        let result = unsafe { WindowsAndMessaging::RegisterClassA(&wc) };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_class() {
        let result = WinTray::register_class();
        assert_ne!(result, 0);
    }
}
