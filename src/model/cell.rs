use std::fmt;

use super::{Orientation};

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Food(usize),
    Stone(usize),
    Snake(usize, Orientation)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
        Cell::Empty => write!(f, "E"),
        Cell::Food(id) => write!(f, "F{}", id),
        Cell::Stone(id) => write!(f, "S{}", id),
        Cell::Snake(id, orientation) => write!(f, "S({}, {})", id, orientation),
        }
    }
}

