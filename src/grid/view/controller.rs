use std::{thread, time};

use crate::model::{Game, TurnResult, PlayerInput, World};


pub trait View {
    fn read_user_inputs(&mut self) -> Vec<UserAction>;
    fn draw_world(&mut self, world: &World);
}

pub enum UserAction {
    Quit,
    Player(usize, PlayerInput)
}

pub struct Controller<V: View + Sized> {
    pub game: Game,
    pub view: V,
    pub quit_on_game_over: bool,
    pub step_interval: time::Duration
}


impl<V: View + Sized> Controller<V> {
    pub fn new(game: Game, view: V) -> Controller<V> {
        Controller {
            game: game,
            view: view,
            quit_on_game_over: true,
            step_interval: time::Duration::from_millis(1000/2)
        }
    }
    pub fn run_loop(&mut self){
        self.view.read_user_inputs(); // drop any user input
        self.view.draw_world(&self.game.world);
        thread::sleep(time::Duration::from_millis(1000/1));
        let mut directions = vec![PlayerInput::DoNothing; self.game.world.player_count()];
        loop{
            // read world state
            let actions = self.view.read_user_inputs();
            for a in actions {
                match a {
                    UserAction::Quit => return,
                    UserAction::Player(pid, dir) => if pid < directions.len() {directions[pid] = dir}
                }
            }
            // run game step
            self.game.advance(&directions);
            let turn_result = self.game.world.turn_result;
            // Display on screen
            self.view.draw_world(&self.game.world);
            let stop = match turn_result {
                TurnResult::Ok => false,
                TurnResult::Draw => {self.view.draw_world(&self.game.world); self.quit_on_game_over},
                TurnResult::GameOver => {self.view.draw_world(&self.game.world); self.quit_on_game_over}
            };
            if stop {
                return
            }
            thread::sleep(self.step_interval);
        } 
    }
}

