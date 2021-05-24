use bindings::Windows::Win32::{
    System::SystemServices::{self, CHAR, HINSTANCE, LRESULT, PSTR},
    UI::{
        MenusAndResources::HICON,
        Shell::{
            self, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAA, NOTIFYICONDATAA_0,
        },
        WindowsAndMessaging::{
            self, HWND, LPARAM, MSG, WM_APP, WM_LBUTTONDOWN, WM_QUIT, WM_RBUTTONDOWN, WNDCLASSA,
            WPARAM,
        },
    },
};
use std::{
    mem, ptr,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};
use windows::Guid;

use crate::TrayIcon;

const ICON_ID: u32 = WM_APP + 1;

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

fn get_msg() {
    let mut msg = MSG::default();
    unsafe {
        loop {
            WindowsAndMessaging::GetMessageA(&mut msg, None, 0, 0);
            if msg.message == WM_QUIT {
                break;
            }

            WindowsAndMessaging::TranslateMessage(&mut msg);
            WindowsAndMessaging::DispatchMessageA(&mut msg);
        }
    }
}

pub struct WinTray {
    nid: NOTIFYICONDATAA,
}

impl WinTray {
    fn register_class(name: &String, instance: &HINSTANCE) {
        let wca = WNDCLASSA {
            lpszClassName: PSTR(name.as_bytes().as_ptr() as _),
            lpfnWndProc: Some(window_proc),
            hInstance: *instance,
            ..Default::default()
        };
        let res = unsafe { WindowsAndMessaging::RegisterClassA(&wca) };
        debug_assert_ne!(res, 0);
    }

    fn create_window(name: &String, instance: &HINSTANCE) -> HWND {
        let hwnd = unsafe {
            WindowsAndMessaging::CreateWindowExA(
                Default::default(),
                PSTR(name.as_bytes().as_ptr() as _),
                PSTR::default(),
                Default::default(),
                0,
                0,
                0,
                0,
                None,
                None,
                *instance,
                ptr::null_mut(),
            )
        };
        debug_assert_ne!(hwnd.0, 0);

        hwnd
    }

    fn notify_icon(hwnd: &HWND) -> NOTIFYICONDATAA {
        let mut nid = NOTIFYICONDATAA {
            cbSize: mem::size_of::<NOTIFYICONDATAA>() as u32,
            hWnd: *hwnd,
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

        nid
    }
}

impl TrayIcon for WinTray {
    fn new() -> Self {
        let (tx, rx): (Sender<NOTIFYICONDATAA>, Receiver<NOTIFYICONDATAA>) = mpsc::channel();

        thread::spawn(move || {
            let name = format!("{} ({})", "Tray", ICON_ID);
            let instance = unsafe { SystemServices::GetModuleHandleA(None) };
            debug_assert_ne!(instance.0, 0);

            WinTray::register_class(&name, &instance);
            let hwnd = WinTray::create_window(&name, &instance);
            let nid = WinTray::notify_icon(&hwnd);

            tx.send(nid).unwrap();

            get_msg();
        });

        let nid = rx.recv().unwrap();

        WinTray { nid }
    }
}
