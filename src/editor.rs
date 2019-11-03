//! editor state. controls operations such as reading and writing text.
#![allow(unused_variables, dead_code)]

// TODO: Make the write function erase the current selection before beginning a write

use std::collections::VecDeque;

/// Information for a particular character cell.
/// Contains color values and other metadata
#[derive(Copy, Clone)]
pub struct CharCel {
    pub char: char,
    fg_on: bool,
    bg_on: bool,
    fg: u16,
    bg: u16,
}

impl Default for CharCel {
    fn default() -> Self {
        return Self {
            char: '0',
            fg_on: false,
            bg_on: false,
            fg: 0,
            bg: 0,
        };
    }
}

impl From<char> for CharCel {
    fn from(a: char) -> CharCel {
        let mut x = Self::default();
        x.char = a;
        return x;
    }
}

// TODO: create a trait for operations on a grid
type Grid = Vec<Vec<CharCel>>;

/// Very simple vector implementation
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2(pub i32, pub i32);
impl Vector2 {
    /// Add two vectors together
    pub fn add(&self, a: impl Into<Self>) -> Self {
        let a = a.into();
        Self(self.0 + a.0, self.1 + a.1)
    }
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

impl From<&Vector2> for Vector2 {
    fn from(a: &Vector2) -> Vector2 {
        a.clone()
    }
}

impl Eq for Vector2 {}

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vector2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            std::cmp::Ordering::Equal
        } else if self.1 < other.1 || (self.1 == other.1 && self.0 < other.0) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from((a, b): (i32, i32)) -> Self {
        Self(a, b)
    }
}

/// Editor state information
pub struct Editor {
    buffer: Grid,
    cursor: Vector2,
    select_start: Option<Vector2>,
    selecting: bool,
}

/// Create an editor for types which implement Into<String>
impl<T: Into<String>> From<T> for Editor {
    fn from(s: T) -> Self {
        let mut editor = Editor::new();
        let s = s.into();

        editor.buffer = s
            .lines()
            .map(|x| x.chars().map(|x| CharCel::from(x)).collect())
            .collect();

        // include the extra new line at the end, .lines omits this.
        if s.ends_with('\n') || s.ends_with("\r\n") {
            editor.buffer.push(Vec::new());
        }

        editor
    }
}

impl Editor {
    /// Create a new editor with the default options
    pub fn new() -> Self {
        return Self {
            buffer: Grid::new(),
            cursor: Vector2(0, 0),
            select_start: None,
            selecting: false,
        };
    }

    pub fn cursor_pos(&self) -> Vector2 {
        self.cursor
    }

    /// Move the cursor towards the given vector
    ///
    /// # Returns
    /// A vector storing the position of the cursor after clamping it to valid coordinates
    pub fn move_cursor(&mut self, direction: impl Into<Vector2>) -> Vector2 {
        let new_pos = self.clamp_vector(self.cursor.add(&direction.into()));
        self.cursor = new_pos;
        return new_pos;
    }

    /// Set the cursor position to a specific coordinate
    ///
    /// # Returns
    /// A vector storing the position of the cursor after clamping it to valid coordinates
    pub fn set_cursor(&mut self, location: impl Into<Vector2>) -> Vector2 {
        let new_pos = self.clamp_vector(location.into());
        self.cursor = new_pos;
        new_pos
    }

    /// clamps a vector to valid grid coordinate
    pub fn clamp_vector(&self, v: Vector2) -> Vector2 {
        let Vector2(x, y) = v;
        let len = self.buffer.len() as i32;

        if y <= 0 {
            self.clamp_to_column(Vector2(x, 0))
        } else if y >= len {
            self.clamp_to_column(Vector2(x, len - 1))
        } else {
            self.clamp_to_column(v)
        }
    }

    /// clamps to a column, the row most be a valid index
    ///
    /// # Panics
    /// row(y) index is outside the buffer length
    fn clamp_to_column(&self, v: Vector2) -> Vector2 {
        let Vector2(x, y) = v;

        let row = self.buffer.get(y as usize);

        // If there is no row at the given y value after clamping, the buffer is probably empty
        if row.is_none() {
            return Vector2(0, 0);
        }

        let row = row.unwrap();
        let len = row.len() as i32;

        if x < 0 {
            Vector2(0, y)
        } else if x >= len {
            Vector2(len, y)
        } else {
            v
        }
    }

    /// begin selecting from the current location of the cursor
    pub fn begin_select(&mut self) {
        self.begin_select_at(self.cursor)
    }

    /// set the selection to start from the given coordinate
    pub fn begin_select_at(&mut self, loc: impl Into<Vector2>) {
        self.select_start = Some(loc.into());
        self.selecting = true;
    }

    /// Clear the currently selected location.
    pub fn clear_selection(&mut self) {
        self.select_start = None;
        self.selecting = false;
    }

    /// copy the selected text
    pub fn copy(&self) -> Option<Vec<CharCel>> {
        if self.selecting {
            Some(self.copy_range(self.select_start.unwrap(), self.cursor))
        } else {
            None
        }
    }

    /// cut the selected text
    pub fn cut(&mut self) -> Option<Vec<CharCel>> {
        if self.selecting {
            self.selecting = false;
            Some(self.cut_range(self.select_start.unwrap(), self.cursor))
        } else {
            None
        }
    }

    /// return the character at location
    pub fn get_cell(&self, location: impl Into<Vector2>) -> Option<CharCel> {
        let location = location.into();

        self.buffer
            .get(location.y() as usize)
            .map(|row| row.get(location.x() as usize).map(|x| x.clone()))
            .and_then(|x| match x {
                Some(x) => Some(x),
                None => None,
            })
    }

    /// Copy the text at location `from` to location `to`
    pub fn copy_range<T: Into<Vector2>>(&self, from: T, to: T) -> Vec<CharCel> {
        use std::cmp::{max, min};
        let (from, to) = (self.clamp_vector(from.into()), self.clamp_vector(to.into()));
        let start = min(from, to);
        let end = max(from, to);

        let mut data = Vec::new();

        let mut position = start.clone();
        while position < end {
            println!("{:?}", position);
            let row = self.buffer.get(position.1 as usize).unwrap();

            // move to the next row when the end of a line has been reached
            if position.0 as usize == row.len() {
                position = position.add(&Vector2(0, 1));
                position.0 = 0;
                data.push(CharCel::from('\n'));
                continue;
            }

            data.push(row.get(position.0 as usize).unwrap().clone());

            // move to the next character
            position = position.add(&Vector2(1, 0));
        }

        data
    }

    /// cut the text from location from, to location to
    pub fn cut_range<T: Into<Vector2>>(&mut self, from: T, to: T) -> Vec<CharCel> {
        use std::cmp::{max, min};

        let (from, to) = (self.clamp_vector(from.into()), self.clamp_vector(to.into()));

        let original_cursor = self.cursor.clone();

        let start = min(from, to);
        let end = max(from, to);

        let mut buffer = VecDeque::<CharCel>::new();
        self.set_cursor(end);

        let mut rows = 0;
        let mut cols = 0;
        while self.cursor > start {
            println!("{:?}, {:?}, {:?}", self.cursor, start, end);
            if let Some(x) = self.delete() {
                match x.char {
                    '\n' => {
                        rows += 1;
                        cols = 0;
                    }
                    _ => cols += 1,
                }
                buffer.push_front(x);
            } else {
                break;
            }
        }

        let original_cursor = if original_cursor > start && original_cursor < end {
            start
        } else if original_cursor.1 == end.1 {
            original_cursor.add(&Vector2(-cols, 0))
        } else if original_cursor > end {
            original_cursor.add(&Vector2(0, -rows))
        } else {
            original_cursor
        };

        // restore the cursor to it's original location after deleting the text
        self.set_cursor(original_cursor);

        Vec::from(buffer)
    }

    /// After writing, the cursor location will be moved `content.len()` characters to the right
    pub fn write(&mut self, content: char) {
        self.write_at(self.cursor.clone(), content);
        self.move_cursor(Vector2(1, 0));

        if content == '\n' {
            // if a newline was inserted, move down to the beginning of next line
            // move the cursor to the beginning of the next line
            self.cursor.0 = 0;
            self.set_cursor((0, self.cursor.1 + 1));
        }
    }

    /// Write a group of cells after `location`
    /// coordinates are provided as (col, row).
    /// the range of valid `col` indices is [0, col.len]
    ///
    /// Upon specifying an out of range coordinate, the location value will be clamped to
    /// the nearest valid position
    pub fn write_at(&mut self, location: impl Into<Vector2>, content: char) {
        let location = self.clamp_vector(location.into());
        let Vector2(x, y) = location;

        // retrieve or create the row at location `y`
        // a row should only need to be created when the vector is empty
        let row = match self.buffer.get_mut(y as usize) {
            Some(row) => row,
            None => {
                self.buffer.push(Vec::new());
                self.buffer.get_mut(y as usize).unwrap()
            }
        };
        let len = row.len();

        // spawn a new line, moving all characters after it to the back
        // otherwise insert a character into the line at the cursor position
        if content == '\n' {
            if (x as usize) >= len {
                self.buffer.insert((y + 1) as usize, Vec::new()); // insert an empty line
            } else {
                // move the content after the cursor to the next line
                let (before, after) = match row.split_at(x as usize) {
                    (before, after) => (before.to_vec(), after.to_vec()),
                };
                *row = before;
                self.buffer.insert((y + 1) as usize, after);
            }
        } else {
            // append or insert a new character depending on the cursor position
            if x >= len as i32 {
                row.push(CharCel::from(content));
            } else {
                row.insert(x as usize, CharCel::from(content));
            }
        }
    }

    /// Delete the cell under the cursor and then shift the cursor one to the left
    ///
    /// # Panics
    /// If `selecting` is true and `select_start` is `none`
    pub fn delete(&mut self) -> Option<CharCel> {
        // delete the entire selection if a current selection is in progress
        if self.selecting {
            self.selecting = false;
            self.cut_range(self.select_start.unwrap(), self.cursor.clone());
        }

        // store the original length of the previous row to jump to when the line below it is deleted
        let previous_row_length = if self.cursor.1 > 0 {
            self.buffer
                .get(self.cursor.1 as usize - 1)
                .map(|x| x.len())
                .unwrap()
        } else {
            0
        };

        // delete the character before the cursor
        let val = self.delete_at(self.cursor);

        let Vector2(_, y) = self.cursor;

        // a line has been deleted, move to the previous line
        if self.cursor.0 == 0 && y >= 1 {
            self.set_cursor((previous_row_length as i32, y - 1));
        } else {
            // otherwise move one character to the left
            self.move_cursor((-1, 0));
        }

        val
    }

    /// Delete the cell at `location` it it exists
    pub fn delete_at(&mut self, location: impl Into<Vector2>) -> Option<CharCel> {
        let Vector2(x, y) = self.clamp_vector(location.into());

        if let Some(row) = self.buffer.get_mut(y as usize) {
            if x == 0 && y >= 1 {
                let mut x = self.buffer.remove(y as usize);
                // append the current line to the previous line
                self.buffer
                    .get_mut((y - 1) as usize)
                    .unwrap()
                    .append(&mut x);
                return Some(CharCel::from('\n'));
            } else if x != 0 && (x as usize) < row.len() {
                return Some(row.remove((x - 1) as usize));
            } else if x != 0 && row.len() != 0 {
                // if the cursor is in a location greater than the last location in the line
                // delete the last element in the buffer
                return Some(row.remove(row.len() - 1));
            }
        }

        None
    }
}

/// Return the contents of the buffer as a string
impl std::fmt::Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: String = self
            .buffer
            .iter()
            .map(|x| x.iter().map(|x| x.char).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;

    const TEST_STRING: &'static str = include_str!("../resources/sample_text.txt");

    #[test]
    fn test_vector_cmp() {
        let test_cases = vec![
            (0, 0, 1, 1, Ordering::Less),
            (10, 0, 0, 0, Ordering::Greater),
            (10, 10, 10, 10, Ordering::Equal),
            (10, 10, 11, 0, Ordering::Greater),
        ];

        for (x1, y1, x2, y2, res) in test_cases {
            assert_eq!(Vector2(x1, y1).cmp(&Vector2(x2, y2)), res);
        }
    }

    // ensure that cut and copy produce the same results
    #[test]
    fn test_editor_copy_range() {
        let mut editor = Editor::from(TEST_STRING);

        #[rustfmt::skip]
        let test_cases = vec![
            (0, 0, 6, 1),
            (20, 20, 0, 100),
            (100, 29, 3, 40),
            (30, 30, 50, 0),
            (100, 100, 0, 0),
        ];

        for (x, y, x2, y2) in test_cases {
            let copy = editor
                .copy_range((x, y), (x2, y2))
                .iter()
                .map(|x| x.char)
                .collect::<String>();
            let cut = editor
                .cut_range((x, y), (x2, y2))
                .iter()
                .map(|x| x.char)
                .collect::<String>();

            assert_eq!(copy, cut)
        }
    }

    #[test]
    fn test_editor_cursor_movement() {
        let mut editor = Editor::from(TEST_STRING);

        for row in 0..(300 / 5) {
            for col in 0..(300 / 5) {
                let (row, col) = ((row * 5), (col * 5));
                let before_set_cursor = editor.cursor.clone();
                editor.set_cursor((col, row));
                let before_write = editor.cursor.clone();
                editor.write('\0');
                let after_write = editor.cursor.clone();
                if let Some(x) = editor.delete() {
                    if x.char != '\0' {
                        panic!(
                            "\
                             deleted character was not '\\0':\n\
                             col, row: {:?}\nbefore set cursor: {:?},\n\
                             after deletion: {:?},\n\
                             before write: {:?},\n\
                             after write: {:?}",
                            (col, row),
                            before_set_cursor,
                            editor.cursor,
                            before_write,
                            after_write
                        );
                    }
                }
            }
        }

        assert_eq!(
            editor.to_string(),
            TEST_STRING.to_string().replace("\r\n", "\n")
        );
    }

    #[test]
    fn test_editor() {
        let mut test_string = "hello world".to_string();
        let mut editor = Editor::new();

        for i in test_string.chars() {
            editor.write(i);
        }

        assert_eq!(editor.to_string(), test_string);

        // move cursor two to the left and insert a character
        editor.move_cursor(Vector2(-2, 0));
        editor.write('s');

        test_string.insert(test_string.len() - 2, 's');
        assert_eq!(editor.to_string(), test_string);

        // move the cursor out of bounds
        editor.move_cursor((9999, 9999));
        editor.delete();
        test_string.pop();

        assert_eq!(editor.to_string(), test_string);

        // insert a new line at the beginning of the string
        editor.set_cursor((0, 0));
        editor.write('\n');

        assert_eq!(editor.buffer.len(), 2 as usize);

        // a new line was successfully inserted at the beginning of the line
        // moving the contents of the previous line to the next
        assert_eq!(
            editor
                .buffer
                .get(1)
                .unwrap()
                .iter()
                .map(|&x| x.char)
                .collect::<String>(),
            test_string
        );

        // move the cursor to the end of the previous line
        editor.move_cursor((9999, -1));
        editor.write('\n');

        assert_eq!(editor.buffer.len(), 3);

        editor.set_cursor((0, 1));
        editor.delete();

        assert_eq!(editor.buffer.len(), 2);

        // assert that the cursor has move to the previous line
        assert_eq!(editor.cursor.1, 0);

        // move the cursor to the very last line and delete it
        editor.set_cursor((0, 9999));
        editor.delete();

        assert_eq!(editor.to_string(), test_string);
        assert_eq!(editor.buffer.len(), 1);

        // delete all characters in editor
        editor.set_cursor((9999, 9999));
        for i in 0..50 {
            println!("{:?}", editor.cursor);
            editor.delete();
        }
        assert_eq!(editor.to_string(), "");
    }

    #[test]
    // TODO : Consider removing carriage returns within the editor function
    // make sure the output is equal to the input
    // to make this work you have to transform \r\n to \n
    fn test_editor_from_string() {
        let test_cases = vec![
            "please wait warmly",
            "\n\n",
            "\n\nso nanoka",
            "have a good smoke\nmokou\n",
            "hello\nworld",
            TEST_STRING,
            "\r\r\r\r\r\n",
            "\rhome alone on a friday night\n\n\r",
        ];

        for i in test_cases {
            let i = i.replace("\r", "");
            let editor = Editor::from(i.to_string());
            assert_eq!(editor.to_string(), i.to_string());
        }
    }
}
