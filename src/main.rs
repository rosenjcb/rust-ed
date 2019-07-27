use rust_ed::screen::{Screen, Direction};
use crossterm::*;
use std::fs::File;
use std::io::{LineWriter, Write};
use log::{debug, info, warn};
use std::path::Path;
use rust_ed::clipboard::Clipboard;


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
        let mut clipboard = Clipboard::new();
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

fn process_input_event(key_event: InputEvent, screen: &mut Screen, clipboard: &mut Clipboard) -> bool {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => match c {
                    'q' => {
                        //info!("The 'q' key is hit and the program is not listening to input anymore.\n\n");
                        screen.save();
                        return true;
                    },
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
                },
                KeyEvent::Right => {
                    let mut cursor = cursor();
                    cursor.move_right(1);
                },
                KeyEvent::Up => {
                    let mut cursor = cursor();
                    cursor.move_up(1);
                },
                KeyEvent::Down => {
                    let mut cursor = cursor();
                    cursor.move_down(1);
                },
                KeyEvent::ShiftRight => {
                    //let cursor = cursor();
                    //let pos = cursor.pos();
                    //let c = screen.buffer[pos.1 as usize].inner[pos.0 as usize].c;
                    screen.highlight(Direction::Right);
                },
                KeyEvent::ShiftLeft => {
                    //let cursor = cursor();
                    //let pos = cursor.pos();
                    screen.highlight(Direction::Left);
                },
                KeyEvent::Ctrl('a') => {
                    //screen.select_all();
                },
                KeyEvent::Ctrl('c') => {
                    clipboard.inner = String::from(screen.get_highlight());
                },
                KeyEvent::Ctrl('v') => {
                    screen.write(clipboard.inner.as_str());
                }
                KeyEvent::Backspace => {
                    screen.delete();
                },
                _ => {
                        println!("{}", format!("OTHER: {:?}\n\n", k));
                }
            }
        }
        InputEvent::Mouse(_) => {}
        InputEvent::Unsupported(_) => {}
        InputEvent::Unknown => {}
    }
    return false;
}