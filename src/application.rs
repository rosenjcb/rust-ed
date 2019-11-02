use crate::clipboard::Clipboard;

/// handles the main application logic
pub struct Application<T>
where
    T: Clipboard,
{
    clipboard: T,
}

impl<T> Application<T>
where
    T: Clipboard,
{
    /// run the application main loop
    pub fn run(&self) {
        // enter raw mode
        // switch to the alternate screen
        // process event loop
    }
}
