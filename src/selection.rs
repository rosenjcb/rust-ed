use crossterm::{Terminal, terminal, TerminalCursor, cursor, Color, style, StyledObject};
use crate::screen::Direction;
//use crate::coord::Coord;

pub struct Selection {
    cursor: TerminalCursor,
    clipboard: (Option<(u16,u16)>, Option<(u16,u16)>),
    start: Option<(u16, u16)>,
    end: Option<(u16, u16)>,
}

impl Selection {
    pub fn new() -> Self {
        let cursor = cursor();
        //let (content, clipboard) = (String::from(""), String::from(""));
        let clipboard: (Option<(u16, u16)>, Option<(u16, u16)>) = (None, None);
        let start: Option<(u16, u16)> = None;
        let end: Option<(u16, u16)> = None;
        Selection { cursor, clipboard, start, end }
    }

    pub fn push(&mut self, c: char, coord: (u16, u16), dir: Direction) {
        let s = c.to_string();
        let style = style(s.as_str());
        //let highlight = style(s.as_str()).with(Color::Black).on(Color::Yellow);
        //print!("{}", highlight);
        match dir {
            Direction::Right => {
                match self.end {
                    None => {
                        self.start(coord);
                        //self.end = Option(coord);
                        let highlight = style.with(Color::Black).on(Color::Yellow);
                        print!("{}", highlight);
                    },

                    Some(prev) => {
                        if prev.0 == (coord.0 - 1) && prev.1 == coord.1 {
                           //self.end = Option(coord);
                            let highlight = style.with(Color::Black).on(Color::Yellow);
                            print!("{}", highlight);
                        } else if prev == coord {
                            let highlight = style.with(Color::White).on(Color::Black);
                            print!("{}", highlight);
                        } else {
                            self.start(coord);
                            let highlight = style.with(Color::Black).on(Color::Yellow);
                            print!("{}", highlight);
                        }
                    }
                }
            },

            Direction::Left => {
                match self.end {
                    None => {
                        self.start(coord);
                    },

                    Some(prev) => {
                        //if(prev)
                        self.start(coord);
                    }
                }
            },

            _ => {

            }
        }
    }

    fn start(&mut self, coord: (u16, u16)) {
        /*let x: Option<(u16, u16)> = Some(coord);
        self.start = Some(coord);
        self.end = Some(coord);
        self.content.0.x = coord.0;
        self.content.0.y = coord.1;
        self.content.1.x = coord.0;
        self.content.1.y = coord.1;*/
        //self.context = context;
    }
    fn update(&mut self, end: (u16,u16)) {
        //self.end = Some(coord);
    }

    pub fn copy(&mut self) {
        self.clipboard = (self.start,  self.end);
    }

    pub fn paste(&mut self) {
        self.start = self.clipboard.0;
        self.end = self.clipboard.1;
    }

    pub fn cut(&mut self) {

    }
}