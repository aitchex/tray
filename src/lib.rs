use wintray::WinTray;

mod wintray;

#[cfg(target_os = "windows")]
pub type Tray = WinTray;

pub trait TrayIcon {
    fn new() -> Self;
}
