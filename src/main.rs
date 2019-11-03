use rust_ed::application::Application;
use rust_ed::clipboard::OsClipboard;
use rust_ed::editor::Editor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Application::new(
        Editor::from(include_str!("../resources/sample_text.txt")),
        OsClipboard::new()?,
    );

    app.run()?;

    Ok(())
}
