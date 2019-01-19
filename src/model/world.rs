use super::*;

pub use grid::Grid;

pub struct World {
    pub snakes: Vec<Snake>,
    pub grid: Grid<Cell>
}

impl World {
    pub fn with_size(rows: usize, cols: usize) -> World {
        debug_assert!(1 <= rows);
        debug_assert!(2 <= cols);
        let mut grid = Grid::from_elem(Cell::Empty, rows, cols);
        grid.set((0,0), Cell::Snake(0, Orientation::Down));
        grid.set((0,1), Cell::Snake(0, Orientation::Left));
        World {
            snakes: vec![Snake{head: (0,0), tail: (0,1)}],
            grid: grid
        }
    }
    pub fn snake_direction<P: Position + Copy>(&self, pos: P) -> Orientation {
        match self.grid.get(pos) {
            Cell::Snake(_id, orientation) => orientation.clone(),
            cell_should_be_snake => panic!("{}, ({}, {}): should be snake cell", cell_should_be_snake, pos.row(), pos.col())
        }
    }
    pub fn move_vector(&self, direction: Orientation) -> (isize, isize) {
        let vertical = match direction {
            Orientation::Up => -1,
            Orientation::Down => 1,
            _ => 0
        };
        let horizontal = match direction {
            Orientation::Left => -1,
            Orientation::Right => 1,
            _ => 0
        };
        (vertical, horizontal)
        
    }
    pub fn advance(&mut self, direction: Orientation) {
        self.advance_snake(0, direction);
    }
    pub fn advance_snake(&mut self, s: usize, direction: Orientation) {
        let mut snake = self.snakes[s];
        let move_head = self.move_vector(direction);
        let head_pos = ((snake.head.row() as isize + move_head.0) as usize, (snake.head.col() as isize + move_head.1) as usize);
        // TODO: check collision
        self.grid.set(snake.head, Cell::Snake(s, direction));
        snake.head = head_pos;
        self.grid.set(snake.head, Cell::Snake(s, direction));

        let move_tail = self.move_vector(self.snake_direction(snake.tail));
        let tail_pos = ((snake.tail.row() as isize + move_tail.0) as usize, (snake.tail.col() as isize + move_tail.1) as usize);
        let next_direction = self.snake_direction(tail_pos);
        self.grid.set(snake.tail, Cell::Empty);
        snake.tail = tail_pos;
        self.grid.set(snake.tail, Cell::Snake(s, next_direction));
        self.snakes[s] = snake;
    }
    pub fn set_head<P: Position>(&mut self, pos: P, direction: Orientation) {
        let s = 0;
        self.grid.set(self.snakes[s].head, Cell::Empty);
        self.snakes[s].head = tuple_from_position(pos);
        self.grid.set(self.snakes[s].head, Cell::Snake(s, direction));
    }
    pub fn set_body<P: Position>(&mut self, pos: P, direction: Orientation) {
        let s = 0;
        self.grid.set(pos, Cell::Snake(s, direction));
    }
    pub fn set_tail<P: Position>(&mut self, pos: P, direction: Orientation) {
        let s = 0;
        self.grid.set(self.snakes[s].tail, Cell::Empty);
        self.snakes[s].tail = tuple_from_position(pos);
        self.grid.set(self.snakes[s].tail, Cell::Snake(s, direction));
    }
 }
