pub use error::Error;
use wintray::WinTray;

mod error;
mod wintray;

#[cfg(target_os = "windows")]
pub type Tray = WinTray;

pub trait TrayIcon {
    fn new() -> Self;
    fn set_icon<S: AsRef<str>>(&mut self, path: S) -> Result<(), Error>;
    fn set_tooltip<S: AsRef<str>>(&mut self, text: S);
}
