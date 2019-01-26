use super::{World, Orientation, GameEvent};

#[derive(Debug, Clone, Copy)]
pub enum PlayerInput {
    DoNothing,
    Go(Orientation)
}

#[derive(Debug, Clone)]
pub enum TurnResult {
    GameOver(Vec<usize>, Vec<usize>), // (winners, losers)
    Draw, // nobody wins, nobody loses
    Ok // nothing bad happened, game goes on
}

pub struct Game {
    pub world: World,
    iteration: usize,
    orientations: Vec<Orientation>,
    pub lose_on_collision: bool
}

impl Game {
    pub fn new(world: World) -> Game {
        Game {
            world: world,
            iteration: 0,
            orientations: Vec::new(),
            lose_on_collision: true
        }
    }
    pub fn advance(&mut self, directions: &Vec<PlayerInput>) -> TurnResult {
        // make sure all invariants with the world hold
        if self.orientations.len() < self.world.player_count() {
            self.orientations = vec![Orientation::Down; self.world.player_count()];
        }
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
        if self.iteration % 16 == 0 && self.world.available_snacks() < 2 {
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
        if self.lose_on_collision && all_collided {
            return TurnResult::Draw;
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
            return TurnResult::GameOver(winners, losers);
        }
        self.iteration += 1;
        TurnResult::Ok
    }
}

