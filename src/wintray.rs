use crate::{error::Error, Click, TrayIcon};

use bindings::Windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PSTR, PWSTR, WPARAM},
    System::{
        Diagnostics::Debug::{self, ERROR_FILE_NOT_FOUND},
        LibraryLoader,
        SystemServices::CHAR,
    },
    UI::{
        Controls::LR_LOADFROMFILE,
        Shell::{
            self, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_MODIFY, NOTIFYICONDATAA,
            NOTIFYICONDATAA_0,
        },
        WindowsAndMessaging::{
            self, HICON, IMAGE_ICON, MSG, WM_APP, WM_LBUTTONUP, WM_QUIT, WM_RBUTTONUP, WNDCLASSA,
        },
    },
};
use std::{
    cell::RefCell,
    mem, ptr,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};
use windows::{Guid, HSTRING};

const ICON_ID: u32 = WM_APP + 1;

thread_local!(static CLICK_TX: RefCell<Option<Sender<Click>>> = RefCell::new(None));

type Callback = Box<(dyn FnMut() -> () + Send + Sync + 'static)>;

pub struct WinTray {
    nid: NOTIFYICONDATAA,
    callback_tx: Sender<(Click, Callback)>,
}

impl WinTray {
    extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        fn send(click: Click) {
            CLICK_TX.with(|click_tx| {
                if let Some(tx) = click_tx.borrow_mut().as_ref() {
                    tx.send(click).unwrap();
                }
            });
        }

        match msg {
            ICON_ID => match lparam.0 as u32 {
                WM_LBUTTONUP => send(Click::Left),
                WM_RBUTTONUP => send(Click::Right),
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

    fn get_module_handle() -> Result<HINSTANCE, Error> {
        let instance = unsafe { LibraryLoader::GetModuleHandleA(None) };
        if instance.0 == 0 {
            let err = unsafe { Debug::GetLastError() };
            return Err(Error::UnknownError(format!("System error code: {}", err.0)));
        }

        Ok(instance)
    }

    fn register_class(name: &String, instance: &HINSTANCE) -> Result<Receiver<Click>, Error> {
        let (tx, rx) = mpsc::channel();

        CLICK_TX.with(|click_tx| {
            (*click_tx.borrow_mut()) = Some(tx);
        });

        let wca = WNDCLASSA {
            lpszClassName: PSTR(name.as_bytes().as_ptr() as _),
            lpfnWndProc: Some(WinTray::wnd_proc),
            hInstance: *instance,
            ..Default::default()
        };

        let result = unsafe { WindowsAndMessaging::RegisterClassA(&wca) };
        if result == 0 {
            let err = unsafe { Debug::GetLastError() };
            return Err(Error::UnknownError(format!("System error code: {}", err.0)));
        }

        Ok(rx)
    }

    fn create_window(name: &String, instance: &HINSTANCE) -> Result<HWND, Error> {
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
        if hwnd.0 == 0 {
            let err = unsafe { Debug::GetLastError() };
            return Err(Error::UnknownError(format!("System error code: {}", err.0)));
        }

        Ok(hwnd)
    }

    fn notify_icon(hwnd: &HWND) -> Result<NOTIFYICONDATAA, Error> {
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

        let result = unsafe { Shell::Shell_NotifyIconA(NIM_ADD, &mut nid) };
        if !result.as_bool() {
            let err = unsafe { Debug::GetLastError() };
            return Err(Error::UnknownError(format!("System error code: {}", err.0)));
        }

        Ok(nid)
    }

    fn modify_icon(&mut self) -> Result<(), Error> {
        let result = unsafe { Shell::Shell_NotifyIconA(NIM_MODIFY, &mut self.nid) };
        if !result.as_bool() {
            let err = unsafe { Debug::GetLastError() };
            return Err(Error::UnknownError(format!("System error code: {}", err.0)));
        }

        Ok(())
    }
}

impl TrayIcon for WinTray {
    fn new() -> Result<Self, Error> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let name = format!("{} ({})", "Tray", ICON_ID);

            let instance = match WinTray::get_module_handle() {
                Ok(instanse) => instanse,
                Err(err) => {
                    tx.send(Err(err)).unwrap();
                    return;
                }
            };

            let click_rx = match WinTray::register_class(&name, &instance) {
                Ok(rx) => rx,
                Err(err) => {
                    tx.send(Err(err)).unwrap();
                    return;
                }
            };

            let (callback_tx, callback_rx): (
                Sender<(Click, Callback)>,
                Receiver<(Click, Callback)>,
            ) = mpsc::channel();

            thread::spawn(move || {
                let mut left_callback: Callback = Box::new(|| {});
                let mut right_callback: Callback = Box::new(|| {});

                loop {
                    let click = click_rx.recv().unwrap();

                    loop {
                        match callback_rx.try_recv() {
                            Ok((clk, f)) => match clk {
                                Click::Left => left_callback = f,
                                Click::Right => right_callback = f,
                            },
                            Err(_) => break,
                        }
                    }

                    match click {
                        Click::Left => left_callback(),
                        Click::Right => right_callback(),
                    }
                }
            });

            let hwnd = match WinTray::create_window(&name, &instance) {
                Ok(hwnd) => hwnd,
                Err(err) => {
                    tx.send(Err(err)).unwrap();
                    return;
                }
            };

            let nid = match WinTray::notify_icon(&hwnd) {
                Ok(nid) => nid,
                Err(err) => {
                    tx.send(Err(err)).unwrap();
                    return;
                }
            };

            tx.send(Ok((nid, callback_tx))).unwrap();

            WinTray::get_msg();
        });

        let (nid, callback_tx) = rx.recv().unwrap()?;

        Ok(WinTray { nid, callback_tx })
    }

    fn set_tooltip<S: AsRef<str>>(&mut self, text: S) -> Result<(), Error> {
        let tooltip = text.as_ref().as_bytes();

        if tooltip.len() > self.nid.szTip.len() {
            let err = format!("Exceeded the {} characters limit", self.nid.szTip.len());
            return Err(Error::OutOfRange(err));
        }

        for i in 0..tooltip.len() {
            self.nid.szTip[i] = CHAR(tooltip[i]);
        }

        self.modify_icon()?;

        Ok(())
    }

    fn set_icon<S: AsRef<str>>(&mut self, path: S) -> Result<(), Error> {
        let hicon = unsafe {
            WindowsAndMessaging::LoadImageW(
                HINSTANCE::NULL,
                PWSTR(HSTRING::from(path.as_ref()).as_wide().as_ptr() as _),
                IMAGE_ICON,
                0,
                0,
                LR_LOADFROMFILE,
            )
        };
        if hicon.0 == 0 {
            let err = unsafe { Debug::GetLastError() };
            match err {
                ERROR_FILE_NOT_FOUND => {
                    let err = String::from("Could not find the specified icon");
                    return Err(Error::NotFound(err));
                }
                _ => return Err(Error::UnknownError(format!("System error code: {}", err.0))),
            }
        }

        self.nid.hIcon = HICON(hicon.0);
        self.modify_icon()?;

        unsafe { WindowsAndMessaging::DestroyIcon(HICON(hicon.0)) };

        Ok(())
    }

    fn on_click<F>(&self, click: Click, callback: F)
    where
        F: 'static + FnMut() -> () + Send + Sync,
    {
        self.callback_tx.send((click, Box::new(callback))).unwrap();
    }
}
