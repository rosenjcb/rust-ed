pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Self {
        Coord { x, y }
    }
}
