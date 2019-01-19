use std::mem;

use crate::model::*;
use grid::Grid;

pub type Player = usize;

pub enum GameEvent {
    Collision(Player, (isize, isize)), // position of collision
    FoodConsumed(Player, usize) // growth value
}

pub struct World {
    pub snakes: Vec<Snake>,
    pub grid: Grid<Cell>,
    events: Vec<GameEvent>,
    pendingGrowth: usize,
    availableSnacks: usize
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
            grid: grid,
            events: Vec::new(),
            pendingGrowth: 1,
            availableSnacks: 0
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
    pub fn place_snack<P: Position + Copy>(&mut self, pos: P, growth_value: usize) -> Result<(), ()> {
        if !match self.grid.get(pos) {
            Cell::Empty => true,
            _ => false
        } {
            return Err(());
        }
        // place snack
        self.grid.set(pos, Cell::Food(growth_value));
        self.availableSnacks += 1;
        Ok(())
    }
    pub fn advance(&mut self, direction: Orientation) -> Vec<GameEvent> {
        self.events.clear();
        self.advance_snake(0, direction);
        let mut events = Vec::new();
        mem::swap(&mut self.events, &mut events);
        events
    }
    fn check_direction(&self, s: usize, dir: Orientation) -> Orientation {
        let head_dir = self.snake_direction(self.snakes[s].head);
        match (dir, head_dir) {
            (Orientation::Left, Orientation::Right) => head_dir,
            (Orientation::Right, Orientation::Left) => head_dir,
            (Orientation::Up, Orientation::Down) => head_dir,
            (Orientation::Down, Orientation::Up) => head_dir,
            _ => dir
        }
    }
    fn advance_snake(&mut self, s: Player, mut direction: Orientation) {
        let mut snake = self.snakes[s];
        direction = self.check_direction(s, direction);
        let move_head = self.move_vector(direction);
        let head_posi = ((snake.head.row() as isize + move_head.0), (snake.head.col() as isize + move_head.1));
        // check collision
        if head_posi.0 < 0 || self.grid.rows() as isize <= head_posi.0 || head_posi.1 < 0 || self.grid.cols() as isize <= head_posi.1 {
            // collision with a wall
            self.events.push(GameEvent::Collision(s, head_posi));
            return
        }
        let head_pos = (head_posi.0 as usize, head_posi.1 as usize);
        match self.grid.get(head_pos) {
            Cell::Food(growth_value) => {
                self.events.push(GameEvent::FoodConsumed(s, *growth_value));
                self.availableSnacks -= 1; 
                self.pendingGrowth += *growth_value},
            Cell::Empty => (),
            Cell::Snake(_,_) => {
                self.events.push(GameEvent::Collision(s, head_posi));
                return;
            },
            Cell::Stone(_) => {
                self.events.push(GameEvent::Collision(s, head_posi));
                return;
            }
        };
        
        // update body segment with picked direction
        self.grid.set(snake.head, Cell::Snake(s, direction));
        // update head position
        snake.head = head_pos;
        self.grid.set(snake.head, Cell::Snake(s, direction));

        // update tail position if no longer growing
        if self.pendingGrowth == 0 {
            let move_tail = self.move_vector(self.snake_direction(snake.tail));
            let tail_pos = ((snake.tail.row() as isize + move_tail.0) as usize, (snake.tail.col() as isize + move_tail.1) as usize);
            let next_direction = self.snake_direction(tail_pos);
            debug_assert!(match self.grid.get(tail_pos) {Cell::Snake(..) => true, _ => false});
            self.grid.set(snake.tail, Cell::Empty);
            snake.tail = tail_pos;
            self.grid.set(snake.tail, Cell::Snake(s, next_direction));
        } else {
            self.pendingGrowth -= 1;
        }
        self.snakes[s] = snake;
        debug_assert!(match self.grid.get(self.snakes[s].head) {Cell::Snake(ss, _) => *ss == s, _ => false});
        debug_assert!(self.is_head(s, self.snakes[s].head));
        debug_assert!(match self.grid.get(self.snakes[s].tail) {Cell::Snake(ss, _) => *ss == s, _ => false});
        debug_assert!(self.is_tail(s, self.snakes[s].tail));
    }
    pub fn is_head<P: Position>(&self, snake: Player, pos: P) -> bool {
        self.snakes[snake].head == tuple_from_position(pos)
    }
    pub fn is_body<P: Position + Copy>(&self, snake: Player, pos: P) -> bool {
        match self.grid.get(pos) {
            Cell::Snake(s, _) => *s == snake && !self.is_head(snake, pos) && !self.is_tail(snake, pos),
            _ => false
        }
    }
    pub fn is_tail<P: Position>(&self, snake: Player, pos: P) -> bool {
        self.snakes[snake].tail == tuple_from_position(pos)
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
