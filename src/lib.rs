pub use error::Error;
use wintray::WinTray;

mod error;
mod wintray;

#[cfg(target_os = "windows")]
pub type Tray = WinTray;

pub trait TrayIcon {
    fn new() -> Self;
    fn set_icon(&mut self, path: &str) -> Result<(), Error>;
    fn set_tooltip(&mut self, text: &str);
}
