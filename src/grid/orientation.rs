use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::Up => write!(f, "Up"),
            Orientation::Down => write!(f, "Down"),
            Orientation::Left => write!(f, "Left"),
            Orientation::Right => write!(f, "Right"),
        }
    }
}

