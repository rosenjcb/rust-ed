// TODO: more informative errors, could wrap an underlying type
//! temporary text buffer
use clipboard::{ClipboardContext, ClipboardProvider};
use std::cell::RefCell;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Copy,
    Paste,
    Os(&'static str, Box<dyn std::error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Copy => "failed to copy".to_string(),
                Error::Paste => "failed to paste".to_string(),
                Error::Os(t, e) => format!("{}: {}", t, e),
            }
        )
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// The clipboard is responsible for storing and retrieving a buffer of characters
pub trait Clipboard {
    /// Return the contents of the clipboard as a string
    fn paste(&self) -> Result<String>;

    /// Replace the current stored data in the clipboard with `content`
    fn copy<T>(&mut self, content: T) -> Result<()>
    where
        T: Into<String>;

    /// clear the contents of the clipboard by setting it to an empty string
    fn clear(&mut self) -> Result<()> {
        self.copy("")
    }
}

/// Just  a wrapper around a String for now... maybe it'll hold formatting someday.
/// In memory implementation of a clipboard
///
/// # Errors
/// None of the in memory clipboard operations will return an error
///
pub struct MemoryClipboard {
    pub inner: String,
}

impl MemoryClipboard {
    pub fn new() -> Self {
        let inner = String::from("");
        Self { inner }
    }
}

impl Clipboard for MemoryClipboard {
    fn paste(&self) -> Result<String> {
        return Ok(self.inner.clone());
    }

    fn copy<T>(&mut self, content: T) -> Result<()>
    where
        T: Into<String>,
    {
        self.inner = content.into();
        Ok(())
    }
}

/// Use the operating systems clipboard to copy and paste data
///
/// # Errors
/// If an error occurs a `clipboard::Error::Os(e)` will be returned with the underlying cause
///
pub struct OsClipboard {
    ctx: RefCell<ClipboardContext>,
}

impl OsClipboard {
    pub fn new() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let ctx: ClipboardContext = ClipboardProvider::new()?;
        Ok(Self {
            ctx: RefCell::new(ctx),
        })
    }
}

impl Clipboard for OsClipboard {
    fn paste(&self) -> Result<String> {
        self.ctx
            .borrow_mut()
            .get_contents()
            .map_err(|e| Error::Os("error pasting: ", e))
    }

    fn copy<T>(&mut self, content: T) -> Result<()>
    where
        T: Into<String>,
    {
        self.ctx
            .borrow_mut()
            .set_contents(content.into())
            .map_err(|e| Error::Os("error copying: ", e))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //  const TEST_DATA: &'static str = "hello world こんにちは世界";
    const TEST_DATA: &'static str = include_str!("../resources/sample_text.txt");

    #[test]
    fn test_memory_clipboard() {
        let mut clipboard = MemoryClipboard::new();
        clipboard.copy(TEST_DATA).unwrap();
        assert_eq!(TEST_DATA, clipboard.paste().unwrap());
    }

    #[test]
    fn test_os_clipboard() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut clipboard = OsClipboard::new()?;

        // save your previous clipboard content
        let old = clipboard.paste()?;

        clipboard.copy(TEST_DATA)?;
        assert_eq!(TEST_DATA, clipboard.paste()?);

        // restore original content
        clipboard.copy(old)?;

        Ok(())
    }
}
