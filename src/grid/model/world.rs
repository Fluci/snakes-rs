use std::mem;

use rand::Rng;

use super::{Grid, Cell, Snake, tuple_from_position, Position, Orientation};

pub type Player = usize;

#[derive(Clone)]
pub enum GameEvent {
    Collision(Player, (isize, isize)), // position of collision
    FoodConsumed(Player, usize) // growth value
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnResult {
    GameOver,
    Draw, // nobody wins, nobody loses
    Ok // nothing bad happened, game goes on
}

#[derive(Clone)]
pub struct World {
    pub snakes: Vec<Snake>,
    pub grid: Grid<Cell>,
    events: Vec<GameEvent>,
    available_snacks: usize,
    pub wall_collision: bool,
    pub turn_result: TurnResult,
    pub winners: Vec<usize>,
    pub losers: Vec<usize>
}

impl World {
    pub fn new(rows: usize, cols: usize) -> World {
        debug_assert!(2 <= rows);
        debug_assert!(1 <= cols);
        World {
            snakes: Vec::new(),
            grid: Grid::from_elem(Cell::Empty, rows, cols),
            events: Vec::new(),
            available_snacks: 0,
            wall_collision: false,
            turn_result: TurnResult::Ok,
            winners: Vec::new(),
            losers: Vec::new()
        }
    }
    pub fn add_snake(&mut self, (head_row, head_col): (usize, usize), direction: Orientation) -> Result<(), ()> {
        debug_assert!(head_row < self.grid.rows());
        debug_assert!(head_col < self.grid.cols());
        if direction == Orientation::Up && head_row >= self.grid.rows()-1 {
            return Err(());
        }
        if direction == Orientation::Down && head_row <= 0 {
            return Err(());
        }
        if direction == Orientation::Left && head_col >= self.grid.cols()-1 {
            return Err(());
        }
        if direction == Orientation::Right && head_col <= 0{
            return Err(());
        }
        let (tail_row, tail_col) = match direction {
            Orientation::Up => (head_row+1, head_col),
            Orientation::Down => (head_row-1, head_col),
            Orientation::Left => (head_row, head_col+1),
            Orientation::Right => (head_row, head_col-1)
        };
        if match self.grid.get((head_row, head_col)) {Cell::Empty => false, _ => true} {
            return Err(());
        }
        if match self.grid.get((tail_row, tail_col)) {Cell::Empty => false, _ => true} {
            return Err(());
        }
        self.snakes.push(Snake::new((head_row, head_col), (tail_row, tail_col)));
        self.grid.set((head_row, head_col), Cell::Snake(self.snakes.len()-1, direction));
        self.grid.set((tail_row, tail_col), Cell::Snake(self.snakes.len()-1, direction));
        Ok(())
    }
    pub fn available_snacks(&self) -> usize {
        self.available_snacks
    }
    pub fn player_count(&self) -> usize {
        self.snakes.len()
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
    pub fn place_snack_randomly(&mut self, growth_value: usize) -> Result<(), ()> {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let row = rng.gen_range(0, self.grid.rows());
            let col = rng.gen_range(0, self.grid.cols());
            match self.place_snack((row, col), growth_value) {
                Ok(_) => return Ok(()),
                _ => ()
            };
        }
        return Err(());
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
        self.available_snacks += 1;
        Ok(())
    }
    pub fn advance(&mut self, directions: &Vec<Orientation>) -> Vec<GameEvent> {
        self.events.clear();
        for i in 0..self.snakes.len() {
            self.advance_snake(i, directions[i]);
        }
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
        let mut head_posi = ((snake.head.row() as isize + move_head.0), (snake.head.col() as isize + move_head.1));
        if !self.wall_collision {
            head_posi.0 = (self.grid.rows() as isize + head_posi.0) % self.grid.rows() as isize;
            head_posi.1 = (self.grid.cols() as isize + head_posi.1) % self.grid.cols() as isize;
        }
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
                self.available_snacks -= 1; 
                snake.pending_growth += *growth_value},
            Cell::Empty => (),
            Cell::Snake(os,_) => {
                self.events.push(GameEvent::Collision(s, head_posi));
                // if the other snake was already moved, and we collide with its head, it's also a
                // collision for the other snake
                if self.is_head(*os, head_pos) && *os < s {
                    self.events.push(GameEvent::Collision(*os, (snake.head.0 as isize, snake.head.1 as isize)));
                }
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
        if snake.pending_growth == 0 {
            let move_tail = self.move_vector(self.snake_direction(snake.tail));
            // mod calculation for !wall_collision
            let tail_pos = (((self.grid.rows() + snake.tail.row()) as isize + move_tail.0) as usize % self.grid.rows(), ((self.grid.rows() + snake.tail.col()) as isize + move_tail.1) as usize % self.grid.rows());
            let next_direction = self.snake_direction(tail_pos);
            debug_assert!(match self.grid.get(tail_pos) {Cell::Snake(..) => true, _ => false});
            self.grid.set(snake.tail, Cell::Empty);
            snake.tail = tail_pos;
            self.grid.set(snake.tail, Cell::Snake(s, next_direction));
        } else {
            snake.pending_growth -= 1;
            snake.length += 1;
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
