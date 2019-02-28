use std::fmt;

use super::{Orientation};

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Food(usize), // growth value
    Stone,
    Snake(usize, Orientation)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
        Cell::Empty => write!(f, "E"),
        Cell::Food(growth_value) => write!(f, "F{}", growth_value),
        Cell::Stone => write!(f, "S"),
        Cell::Snake(id, orientation) => write!(f, "S({}, {})", id, orientation),
        }
    }
}

