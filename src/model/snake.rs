use std::fmt;

use super::Position;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct Snake {
    pub head: Pos,
    pub tail: Pos,
    pub pendingGrowth: usize
}

impl Snake {
    pub fn new(head: Pos, tail: Pos) -> Snake {
        Snake {
            head: head,
            tail: tail,
            pendingGrowth: 0
        }
    }
}

impl fmt::Display for Snake{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Snake(({},{}), ({},{}))", 
               self.head.row(), self.head.col(), self.tail.row(), self.tail.col())
    }
}

