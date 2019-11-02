//! handles rendering an editor state

use crate::editor::{Editor, Vector2};

/// contains parameters for rendering
#[derive(Clone)]
pub struct RenderOpts {
    // the point at which rendering begins
    view: Rect,
}

#[derive(Clone)]
pub struct Rect {
    location: Vector2,
    width: i32,
    height: i32,
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
                } else if x >= 0 {
                    // conclude this loop and continue to the next line when no characters are found
                    break;
                } else {
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
