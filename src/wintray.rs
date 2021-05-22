use std::{mem, ptr};

use bindings::Windows::Win32::{
    System::SystemServices::{self, CHAR, LRESULT, PSTR},
    UI::{
        MenusAndResources::HICON,
        Shell::{
            self, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAA, NOTIFYICONDATAA_0,
        },
        WindowsAndMessaging::{
            self, HWND, LPARAM, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WNDCLASSA, WPARAM,
        },
    },
};
use windows::Guid;

use crate::TrayIcon;

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

pub struct WinTray {
    nid: NOTIFYICONDATAA,
}

impl TrayIcon for WinTray {
    fn new() -> Self {
        let class_name = format!("{} {}", "Tray", ICON_ID);
        let instance = unsafe { SystemServices::GetModuleHandleA(None) };

        let wca = WNDCLASSA {
            lpszClassName: PSTR(class_name.as_bytes().as_ptr() as _),
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            ..Default::default()
        };
        let res = unsafe { WindowsAndMessaging::RegisterClassA(&wca) };
        debug_assert_ne!(res, 0);

        let hwnd = unsafe {
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
        debug_assert_ne!(hwnd.0, 0);

        let mut nid = NOTIFYICONDATAA {
            cbSize: mem::size_of::<NOTIFYICONDATAA>() as u32,
            hWnd: hwnd,
            uID: ICON_ID,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: ICON_ID,
            hIcon: HICON::default(),
            szTip: [CHAR::default(); 128],
            dwState: 0,
            dwStateMask: 0,
            szInfo: [CHAR::default(); 256],
            Anonymous: NOTIFYICONDATAA_0 { uTimeout: 0 },
            szInfoTitle: [CHAR::default(); 64],
            dwInfoFlags: 0,
            guidItem: Guid::default(),
            hBalloonIcon: HICON::default(),
        };

        let res = unsafe { Shell::Shell_NotifyIconA(NIM_ADD, &mut nid) };
        debug_assert_ne!(res.0, 0);

        WinTray { nid }
    }
}
