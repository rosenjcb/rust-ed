//! handles rendering an editor state

use crate::editor::{Editor, Vector2};

/// contains parameters for rendering
#[derive(Clone, Copy, Debug)]
pub struct RenderOpts {
    pub view: Rect,
}

impl Default for RenderOpts {
    fn default() -> Self {
        Self {
            view: Rect {
                location: Vector2(0, 0),
                width: 0,
                height: 0,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub location: Vector2,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    /// return the area of a rectangle
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    pub fn x(&self) -> i32 {
        self.location.x()
    }
    pub fn y(&self) -> i32 {
        self.location.y()
    }

    pub fn contains(&self, p: Vector2) -> bool {
        return (p.x() >= self.location.x() && p.x() < self.location.x() + self.width) &&
            (p.y() >= self.location.y() && p.y() < self.location.y() + self.height)
    }
}

pub trait Renderer {
    type Output;
    fn render(&self, editor: &Editor, opts: RenderOpts) -> Self::Output;
}

/// renders an editor state to a string
pub struct StringRenderer();

impl Renderer for StringRenderer {
    type Output = String;

    fn render(&self, editor: &Editor, opts: RenderOpts) -> Self::Output {
        // draw the rectangle
        let mut screen: String = String::with_capacity(opts.view.area() as usize);

        for y in opts.view.y()..opts.view.y() + opts.view.height {
            for x in opts.view.x()..opts.view.x() + opts.view.width {
                if let Some(cell) = editor.get_cell((x, y)) {
                    screen.push(cell.char);
                }
                /* else if x >= 0 {
                    // conclude this loop and continue to the next line when no characters are found
                     break;
                } */
                else {
                    screen.push(' ');
                }
            }
            screen.push('\n')
        }

        screen
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;
    //    const SAMPLE_TEXT: &'static str = include_str!("../resources/sample_text.txt");

    //    #[test]
    //    fn test_string_renderer() {
    //        let editor = Editor::from(SAMPLE_TEXT);
    //        let renderer = StringRenderer();
    //
    //        let renderOpts = RenderOpts {
    //            view: Rect {
    //                location: Vector2(0, 0),
    //                width: 180,
    //                height: 25,
    //            }
    //        };
    //
    //        let text = renderer.render(&editor, renderOpts);
    //        panic!("\n{}", text);
    //    }
}
