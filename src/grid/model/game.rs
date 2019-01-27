use super::{World, TurnResult, Orientation, GameEvent};

#[derive(Debug, Clone, Copy)]
pub enum PlayerInput {
    DoNothing,
    Go(Orientation)
}

#[derive(Clone)]
pub struct Game {
    pub world: World,
    iteration: usize,
    orientations: Vec<Orientation>,
    pub lose_on_collision: bool,
    pub max_snacks: usize
}

impl Game {
    pub fn new(world: World) -> Game {
        Game {
            world: world,
            iteration: 0,
            orientations: Vec::new(),
            lose_on_collision: true,
            max_snacks: 2
        }
    }
    pub fn iteration(&self) -> usize {
        self.iteration
    }
    pub fn advance(&mut self, directions: &Vec<PlayerInput>) {
        // make sure all invariants with the world hold
        if self.orientations.len() < self.world.player_count() {
            self.orientations = vec![Orientation::Down; self.world.player_count()];
        }
        self.world.winners.clear();
        self.world.losers.clear();
        // read world state for default behavior
        for i in 0..self.world.player_count() {
            self.orientations[i] = self.world.snake_direction(self.world.snakes[i].head);
        }
        // apply user choice
        for i in 0..self.world.player_count() {
            self.orientations[i] = match directions[i] {
                PlayerInput::DoNothing => self.orientations[i],
                PlayerInput::Go(orient) => orient
            }
        }
        // Physics
        let events = self.world.advance(&self.orientations);
        // Game events
        // TODO: could be more exciting ...
        if self.iteration % 16 == 0 && self.world.available_snacks() < self.max_snacks {
            self.world.place_snack_randomly((2*self.iteration)%3+1).unwrap();
        }
        // Apply game rules
        let mut players_collided = vec![false; self.world.player_count()];
        for e in events {
            match e {
                GameEvent::Collision(s, _) => players_collided[s] = true,
                _ => ()
            };
        }
        let all_collided = players_collided.iter().fold(true, |sum, x| sum && *x);
        let some_collided = players_collided.iter().fold(false, |sum, x| sum || *x);
        if self.lose_on_collision && all_collided && self.world.snakes.len() > 1 {
            self.world.turn_result = TurnResult::Draw;
            return
        }
        else if self.lose_on_collision && some_collided {
            let mut winners = Vec::new();
            let mut losers = Vec::new();
            for i in 0..self.world.player_count() {
                if !players_collided[i] {
                    winners.push(i);
                } else {
                    losers.push(i);
                }
            }
            self.world.winners = winners;
            self.world.losers = losers;
            self.world.turn_result = TurnResult::GameOver;
            return
        }
        // winn if there's hardly any space left
        if self.world.snakes.len() == 1 && self.world.snakes[0].length >= (self.world.grid.rows()-1) * (self.world.grid.cols()-1){
            self.world.winners.push(0);
            self.world.turn_result = TurnResult::GameOver;
            return 
        }
        self.iteration += 1;
        self.world.turn_result = TurnResult::Ok
    }
}

