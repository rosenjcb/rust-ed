use crossterm::*;
use log::{debug, info, warn};
use rust_ed::clipboard::{Clipboard, MemoryClipboard};
use rust_ed::screen::{Direction, Screen};
use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

fn main() {
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        stderrlog::new().module(module_path!()).init().unwrap();
        //let color = color();
        let terminal = terminal();
        let cursor = cursor();
        terminal.set_size(80, 24);
        terminal.clear(ClearType::All);
        //cursor.pos();
        //cursor.goto(0,0);
        let input = input();
        input.disable_mouse_mode().unwrap();
        let mut sync_stdin = input.read_sync();

        cursor.show().expect("Cannot display cursor");
        let mut screen = Screen::new(80, 24);
        let mut clipboard = MemoryClipboard::new();
        //let mut selection = Selection::new(&screen);
        loop {
            let event = sync_stdin.next();

            if let Some(key_event) = event {
                if process_input_event(key_event, &mut screen, &mut clipboard) {
                    break;
                }
            }
            //screen.render();
        }
    }
}

fn process_input_event(
    key_event: InputEvent,
    screen: &mut Screen,
    clipboard: &mut impl Clipboard,
) -> bool {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => match c {
                    's' => {
                        //info!("The 'q' key is hit and the program is not listening to input anymore.\n\n");
                    }
                    'l' => {
                        let path = Path::new("intro.txt");
                        screen.load(path);
                    }
                    _ => {
                        //let cursor = cursor();
                        screen.write(c.to_string().as_str());
                        //screen.write(cursor.pos().0, cursor.pos().1, c);
                    }
                },
                KeyEvent::Left => {
                    let mut cursor = cursor();
                    cursor.move_left(1);
                    //warn!("test");
                }
                KeyEvent::Right => {
                    let mut cursor = cursor();
                    cursor.move_right(1);
                }
                KeyEvent::Up => {
                    let mut cursor = cursor();
                    cursor.move_up(1);
                }
                KeyEvent::Down => {
                    let mut cursor = cursor();
                    cursor.move_down(1);
                }
                KeyEvent::ShiftRight => {
                    //let cursor = cursor();
                    //let pos = cursor.pos();
                    //let c = screen.buffer[pos.1 as usize].inner[pos.0 as usize].c;
                    screen.highlight(Direction::Right);
                }
                KeyEvent::ShiftLeft => {
                    //let cursor = cursor();
                    //let pos = cursor.pos();
                    screen.highlight(Direction::Left);
                }
                KeyEvent::Ctrl('a') => {
                    //screen.select_all();
                }
                KeyEvent::Ctrl('c') => {
                    clipboard.copy(String::from(screen.get_highlight()));
                }
                KeyEvent::Ctrl('v') => {
                    screen.write(clipboard.paste().unwrap_or("".to_string()).as_str());
                }
                KeyEvent::Ctrl('s') => {
                    screen.save();
                }
                KeyEvent::Backspace => {
                    screen.delete();
                }
                KeyEvent::Esc => {
                    return true;
                }
                _ => {
                    //println!("{}", format!("OTHER: {:?}\n\n", k));
                }
            }
        }
        InputEvent::Mouse(_) => {}
        InputEvent::Unsupported(_) => {}
        InputEvent::Unknown => {}
    }
    return false;
}
