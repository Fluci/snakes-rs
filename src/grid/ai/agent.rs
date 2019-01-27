use crate::model::{Game, PlayerInput};

pub trait Agent {
    fn decide(&mut self, game: &Game) -> PlayerInput;
}

