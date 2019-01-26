use super::Position;

pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T: Clone> Grid<T> {
    pub fn from_elem(elem: T, rows: usize, cols: usize) -> Grid<T> {
        let size = rows*cols;
        Grid {
            rows: rows, 
            cols: cols,
            data: vec![elem; size]
        }
    }
}

impl<T> Grid<T> {
    pub fn index<P: Position>(&self, pos: P) -> usize {
        debug_assert!(pos.row() < self.rows && pos.col() < self.cols);
        self.cols * pos.row() + pos.col()
    }
    pub fn set<P: Position>(&mut self, pos: P, value:T ) {
        let i = self.index(pos);
        self.data[i] = value;
    }
    pub fn get<P: Position>(&self, pos: P) -> &T {
        let i = self.index(pos);
        &self.data[i]
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
}

