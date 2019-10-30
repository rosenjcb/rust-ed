use crate::screen::Direction;
use crossterm::{cursor, style, Color, StyledObject, Terminal, TerminalCursor};
//use crate::coord::Coord;

pub struct Selection {
    cursor: TerminalCursor,
    clipboard: String,
    //start: Option<(u16, u16)>,
    //end: Option<(u16, u16)>,
}

impl Selection {
    pub fn new() -> Self {
        let cursor = cursor();
        //let (content, clipboard) = (String::from(""), String::from(""));
        //let clipboard: (Option<(u16, u16)>, Option<(u16, u16)>) = (None, None);
        let clipboard = String::from("");
        //let start: Option<(u16, u16)> = None;
        //let end: Option<(u16, u16)> = None;
        Selection { cursor, clipboard }
    }

    /*pub fn push(&mut self, c: char, coord: (u16, u16), dir: Direction) {
        self.text.push(c);
        let s = c.to_string();
        let style = style(s.as_str());
        //let highlight = style(s.as_str()).with(Color::Black).on(Color::Yellow);
        //print!("{}", highlight);
    }

    fn start(&mut self, coord: (u16, u16)) {
        *//*let x: Option<(u16, u16)> = Some(coord);
        self.start = Some(coord);
        self.end = Some(coord);
        self.content.0.x = coord.0;
        self.content.0.y = coord.1;
        self.content.1.x = coord.0;
        self.content.1.y = coord.1;*//*
        //self.context = context;
    }
    fn update(&mut self, end: (u16,u16)) {
        //self.end = Some(coord);
    }*/

    pub fn push_string(&mut self, string: &str) {
        self.clipboard = String::from(string);
    }

    pub fn peek_string(&mut self) -> &str {
        return self.clipboard.as_str();
    }

    /*pub fn pop_string(&mut self) -> &str {
        let copy = self.clipboard.as_str();
        self.clipboard.clear();
        return copy;
    }*/
}
