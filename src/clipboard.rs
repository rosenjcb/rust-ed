// TODO: create an implementation of clipboard that interfaces with the operating system

/// The clipboard trait is responsible for storing and retrieving a buffer of characters
pub trait Clipboard {
    /// Return the contents of the clipboard as a string
    fn paste(&self) -> String;

    /// Replace the current stored data in the clipboard with `content`
    fn copy<T>(&mut self, content: T)
    where
        T: Into<String>;
}

/// Just  a wrapper around a String for now... maybe it'll hold formatting someday.
pub struct MemoryClipboard {
    pub inner: String
}

/// MemoryClipboard is an in memory implementation of the clipboard trait
impl MemoryClipboard {
    pub fn new() -> Self {
        let inner = String::from("");
        Self { inner }
    }
}

impl Clipboard for MemoryClipboard {
    fn paste(&self) -> String {
        return self.inner.clone();
    }

    fn copy<T>(&mut self, content: T)
    where
        T: Into<String>,
    {
        self.inner = content.into();
    }
}
