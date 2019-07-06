//use crossterm::Color;

pub enum Color {
    BLACK,
    WHITE,
}

pub struct GridCell {
    pub c: char,
    pub bg: Color,
    pub fg: Color,
}
