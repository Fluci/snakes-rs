pub trait Position {
    fn row(&self) -> usize;
    fn col(&self) -> usize;
}
pub fn tuple_from_position<P: Position>(other: P) -> (usize, usize) {
    (other.row(), other.col())
}

impl Position for (usize, usize) {
    fn row(&self) -> usize {
        self.0
    } 
    fn col(&self) -> usize {
        self.1
    }
}

