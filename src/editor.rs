//! editor state. controls operations such as reading and writing text.
#![allow(unused_variables, dead_code)]

/// Information for a particular character cell.
/// Contains color values and other metadata
#[derive(Copy, Clone)]
pub struct CharCel {
    char: char,
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
#[derive(Copy, Clone)]
pub struct Vector2(i32, i32);
impl Vector2 {
    /// Add two vectors together
    fn add(&self, a: &Self) -> Self {
        Self(self.0 + a.0, self.1 + a.1)
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
    select_end: Option<Vector2>,
    selecting: bool,
}

impl Editor {
    /// Create a new editor with the default options
    pub fn new() -> Self {
        return Self {
            buffer: Grid::new(),
            cursor: Vector2(0, 0),
            select_start: None,
            select_end: None,
            selecting: false,
        };
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

    /// After writing, the cursor location will be moved `content.len()` characters to the right
    pub fn write(&mut self, content: char) {
        self.write_at(self.cursor.clone(), content);
        self.move_cursor(Vector2(1, 0));

        if content == '\n' {
            // if a newline was inserted, move down to the beginning of next line
            // move the cursor to the beginning of the next line
            self.cursor.0 = 0;
            self.move_cursor((0, self.cursor.1 + 1));
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
    pub fn delete(&mut self) {
        self.delete_at(self.cursor.clone());

        let Vector2(x, y) = self.cursor;

        // a line has been deleted, move to the end of the previous line
        if self.cursor.0 == 0 && y >= 1 {
            let len = self.buffer.get((y - 1) as usize).unwrap().len();
            self.set_cursor(Vector2(len as i32, y - 1));
        } else {
            // otherwise move one character to the left
            self.move_cursor((-1, 0));
        }
    }

    /// Delete the cell at `location` it it exists
    pub fn delete_at(&mut self, location: impl Into<Vector2>) {
        let Vector2(x, y) = self.clamp_vector(location.into());

        if let Some(row) = self.buffer.get_mut(y as usize) {
            if x == 0 && y >= 1 {
                let mut x = self.buffer.remove(y as usize);
                // append the current line to the previous line
                self.buffer
                    .get_mut((y - 1) as usize)
                    .unwrap()
                    .append(&mut x);
            } else if (x as usize) < row.len() {
                row.remove(x as usize);
            } else {
                // if the cursor is in a location greater than the last location in the line
                // delete the last element in the buffer
                row.remove(row.len() - 1);
            }
        }
    }
}

impl std::fmt::Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: String = self.buffer.iter().flatten().map(|x| x.char).collect();
        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    }
}