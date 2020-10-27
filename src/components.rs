use raqote::Path;

pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Drawable {
    pub width: u32,
    pub height: u32,
    pub path: Path,
}
