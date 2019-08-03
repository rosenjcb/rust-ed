//Just  a wrapper around a String for now... maybe it'll hold formatting someday.
pub struct Clipboard {
    pub inner: String
}

impl Clipboard {
    pub fn new() -> Self {
        let inner = String::from("");
        Clipboard { inner }
    }
}