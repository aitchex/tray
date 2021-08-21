pub use error::Error;
use wintray::WinTray;

mod error;
mod wintray;

pub enum Click {
    Left,
    Right,
}

#[cfg(target_os = "windows")]
pub type Tray = WinTray;

pub trait TrayIcon {
    fn new() -> Result<Tray, Error>;
    fn set_tooltip<S: AsRef<str>>(&mut self, text: S) -> Result<(), Error>;
    fn set_icon<S: AsRef<str>>(&mut self, path: S) -> Result<(), Error>;
    fn on_click<F>(&self, click: Click, callback: F)
    where
        F: 'static + FnMut() -> () + Send + Sync;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray() -> Result<(), Error> {
        let mut tray = Tray::new()?;

        tray.set_tooltip("Testing tooltip with a text longer than Lorem Ipsum")?;
        tray.set_tooltip("Lorem Ipsum")?;

        tray.set_icon("res/111.ico")?;
        tray.set_icon("res/222.ico")?;

        tray.on_click(Click::Left, || println!("Left Click"));
        tray.on_click(Click::Right, || println!("Right Click"));

        // std::thread::sleep(core::time::Duration::from_secs(10));

        Ok(())
    }
}
