extern crate gridsnakes;

use gridsnakes::view::{TermionView, View, UserAction};
use gridsnakes::model::{Game, World, PlayerInput, Orientation, TurnResult};
use gridsnakes::ai::{Agent, SpaceExplorer};
use std::{thread, time};

fn main(){
    let size = 10;
    let snakes = 1;
    let mut game = Game::new(World::new(size, size, snakes));
    let mut view = TermionView::new();
    let mut sleep_interval = time::Duration::from_millis(1000/10);
    view.draw_world(&game.world);
    let mut agent = SpaceExplorer::new(6);
    loop {
        let input = vec![agent.decide(&game)];
        game.advance(&input);
        if game.iteration() % 1 == 0 {
            view.draw_world(&game.world);
            thread::sleep(sleep_interval); 
        }
        for e in view.read_user_inputs() {
            match e {
                UserAction::Quit => return,
                UserAction::Player(0, PlayerInput::Go(Orientation::Up)) => (sleep_interval /= 2),
                UserAction::Player(0, PlayerInput::Go(Orientation::Down)) => (sleep_interval *= 2),
                _ => ()
            }
        }
        if game.iteration() > 1000 || game.world.turn_result != TurnResult::Ok {
            break;
        }
    }
}
