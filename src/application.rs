use crate::clipboard::Clipboard;
use crate::editor::{Editor, Vector2};
use crate::renderer::{RenderOpts, Renderer, StringRenderer};

use crossterm::{cursor::MoveTo, input::{InputEvent, KeyEvent, SyncReader}, screen::{self}, terminal::{self}, ExecutableCommand};

use std::io::Write;
use crossterm::input::{MouseEvent, EnableMouseCapture};

/// handles the main application logic
pub struct Application<T>
where
    T: Clipboard,
{
    pub editor: Editor,
    pub clipboard: T,
    pub render_opts: RenderOpts,
    pub exit: bool,
    pub log: String,
}

impl<T> Application<T>
where
    T: Clipboard,
{
    pub fn new(editor: Editor, clipboard: T) -> Application<T> {
        Application {
            editor,
            clipboard,
            render_opts: RenderOpts::default(),
            exit: false,
            log: String::new(),
        }
    }

    /// run the application main loop
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // enter raw mode
        // switch to the alternate screen
        let _alternate = screen::AlternateScreen::to_alternate(true)?;
        // process keyboard events
        let mut reader = SyncReader;

        // enable mouse capture
        std::io::stdout().execute(EnableMouseCapture).unwrap();

        self.render();

        loop {
            if self.exit {
                break Ok(());
            }

            if let Some(event) = reader.next() {
                self.process_event(event);
            }

            // thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    pub fn process_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::Keyboard(event) => self.process_key_event(event),
            InputEvent::Mouse(event) => self.process_mouse_event(event),
            _ => {}
        }
    }

    pub fn process_mouse_event(&mut self, event: MouseEvent) {
        use MouseEvent::*;

        self.log = "Processing mouse event".to_string();

        // convert screen coordinates into editor coordinates
        macro_rules! to_editor_coords {
            ($x:ident, $y:ident) => {
                {
                    let Vector2(x2, y2) = self.render_opts.view.location;
                    ($x + x2, $y + y2)
                }
            }
        }

        match event {
            Press(_, x, y) => {
                let (x, y) = (x as i32, y as i32);
                let (x, y) = to_editor_coords!(x, y);
                self.log = format!("mouse: set cursor location to {}:{}", x, y);
                self.editor.set_cursor((x, y));
                self.render();
            }
            _ => { self.log = "unknown mouse event".to_string()}
        }
    }

    pub fn process_key_event(&mut self, event: KeyEvent) {
        use KeyEvent::*;

        macro_rules! move_view {
            ($x:literal, $y:literal) => {
                self.render_opts.view.location = self.render_opts.view.location.add(Vector2($x, $y));
                self.render();
            };
        }

        macro_rules! move_cursor {
            ($x:literal, $y:literal) => {
                self.editor.move_cursor(($x, $y));
                self.render();
            };
        }

        match event {
            Down => {
                move_cursor!(0, 1);
            }
            Up => {
                move_cursor!(0, -1);
            }
            Right => {
                move_cursor!(1, 0);
            }
            Left => {
                move_cursor!(-1, 0);
            }
            CtrlDown => {
                move_view!(0, 1);
            }
            CtrlUp => {
                move_view!(0, -1);
            }
            CtrlRight => {
                move_view!(1, 0);
            }
            CtrlLeft => {
                move_view!(-1, 0);
            }
            Ctrl('c') => {
                self.exit = true;
                self.render();
            }
            Char(x) => {
                self.editor.write(x);
                self.render();
            }
            Backspace => {
                self.editor.delete();
                self.render();
            }
            Enter => {
                self.editor.write('\n');
                self.render();
            }
            _ => {}
        }
    }

    /// render the screen to crossterm
    pub fn render(&mut self) {
        self.update_view_size().unwrap();

        let text = StringRenderer().render(&self.editor, self.render_opts);

        //        std::fs::write("render_contents.txt", &text).unwrap();

        let mut stdout = std::io::stdout();
        stdout.execute(MoveTo(0, 0)).unwrap();
        write!(&mut stdout, "{}{:?}{}", text, self.render_opts, self.log).unwrap();

        if self.render_opts.view.contains(self.editor.cursor_pos()) {
            // place the cursor over the current character
            let x = self.render_opts.view.x();
            let y = self.render_opts.view.y();

            // obtain the position of the cursor relative to the screen
            let real_x = self.editor.cursor_pos().x() - x;
            let real_y = self.editor.cursor_pos().y() - y;

            stdout.execute(MoveTo(real_x as u16, real_y as u16)).unwrap();
        }
    }

    /// update the view size for the renderer
    pub fn update_view_size(&mut self) -> crossterm::Result<()> {
        let (cols, rows) = terminal::size()?;
        self.render_opts.view.width = cols as i32;
        self.render_opts.view.height = rows as i32 - 1;
        Ok(())
    }
}
