use crate::gridrow::GridRow;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::fs;
use std::path::Path;
use crate::selection::Selection;
use crossterm::{style, Color, Terminal, cursor, terminal, Colorize};
use crate::gridcell::GridCell;
use std::collections::VecDeque;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Screen {
    terminal: Terminal,
    view_loc: u16,
    buffer: Vec<GridRow>,
    selection: Selection,
    highlight: VecDeque<(u16, u16)>,
}

impl Screen {
    pub fn new(x: i32, y: i32) -> Self {
        //let mut buffer: Vec<GridRow> = Vec::with_capacity(usize::from(y));
        let buffer = (0 .. y).map(|_| GridRow::new(x)).collect::<Vec<_>>();
        /*for i in 0..y {
            buffer.push(GridRow::new(x as i32));
        }*/
        let view_loc = 0;
        let terminal = terminal();
        let selection = Selection::new();
        let highlight = VecDeque::new();
        Screen{ terminal, view_loc, buffer, selection, highlight }
    }

    pub fn save(&self){
        let file = File::create("test.txt").expect("Couldn't create file.");
        let mut filewriter = LineWriter::new(file);
        self.buffer.iter().for_each(|row| {
            filewriter.write(row.getline().as_bytes()).expect("Cannot write to buffer");
        });

    }

    pub fn write(&mut self, c: char) {
        let cursor = cursor();
        let pos = cursor.pos();
        self.buffer[cursor.pos().1 as usize].write(cursor.pos().0  as i32, c);
        self.terminal.write(c);
    }

    pub fn highlight(&mut self, dir: Direction) {

        match dir {
            Direction::Right => {
                let cursor = &mut cursor();
                let coord = cursor.pos();
                let c = self.get_char(&coord);

                let prev_coord = &(coord.0 - 1, coord.1);
                let next_coord = &(coord.0 + 1, coord.1);

                if self.highlight.contains(next_coord) {
                    cursor.move_right(1);
                    return;
                }

                if self.highlight.len() == 0 || self.highlight.back().expect("No coord found") == prev_coord {
                    self.highlight.push_back(coord);
                } else {
                    let s: String = self.highlight.iter().map(|selection| self.get_char(selection)).collect();
                    let first = self.highlight.front().expect("No coord found");
                    cursor.goto(first.0, first.1);
                    print!("{}", s);
                    cursor.goto(coord.0, coord.1);
                    self.highlight.clear();
                    self.highlight.push_back(coord);
                }
                let s = c.to_string();
                let highlight = style(s.as_str()).with(Color::Black).on(Color::Yellow);
                print!("{}", highlight);
                self.selection.push(c, coord, dir);
            },

            Direction::Left => {
                let cursor = &mut cursor();
                cursor.move_left(1);
                let coord = cursor.pos();
                let c = self.get_char(&coord);

                let prev_coord = &(coord.0 + 1, coord.1);
                let next_coord = &(coord.0 - 1, coord.1);
                if self.highlight.contains(next_coord) {
                    cursor.move_left(1);
                    return;
                }

                if self.highlight.len() == 0 || self.highlight.front().expect("No coord found") == prev_coord {
                    self.highlight.push_front(coord);
                } else {
                    let s: String = self.highlight.iter().map(|selection| self.get_char(selection)).collect();
                    let first = self.highlight.front().expect("No coord found");
                    cursor.goto(first.0, first.1);
                    print!("{}", s);
                    cursor.goto(coord.0, coord.1);
                    self.highlight.clear();
                    self.highlight.push_front(coord);
                }
                let s = c.to_string();
                let highlight = style(s.as_str()).with(Color::Black).on(Color::Yellow);
                print!("{}", highlight);
                cursor.move_left(1);
                //cursor.goto(coord.0 - 1, coord.1);
                self.selection.push(c, coord, dir);
            },

            _ => {

            }
        }
    }

    /*pub fn cancel_highlight(&mut self, coord: &(u16, u16)) {
        let cursor = cursor();
        let s: String = self.highlight.iter().rev().map(|selection| self.get_char(selection)).collect();
        let last = self.highlight.get(self.highlight.len() - 1).expect("No coord found");
        cursor.goto(last.0, last.1);
        print!("{}", s);
        cursor.goto(coord.0, coord.1);
        self.highlight.clear();
        self.highlight.push_front(*coord);
    }*/

    pub fn get_char(&self, coord: &(u16, u16)) -> char {
        self.buffer[coord.1 as usize].inner[coord.0 as usize]
    }

    pub fn load(&mut self, filepath: &Path) {
        let cursor = cursor();
        let contents = fs::read_to_string(filepath).expect("File not found!");
        cursor.goto(0,0);
        println!("{}", contents);
    }

    //This sucks...
    /*pub fn render(&self) {
        //(0..self.buffer.len()).for_each(|line| self.buffer[line].printline());
        for(index, line) in self.buffer.iter().enumerate() {
            line.printline(self.view_loc + (index as u16));
        }
    }*/
}