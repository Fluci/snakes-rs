extern crate termion;
extern crate rand;
extern crate gridsnakes;

mod controller;
mod termion_view;

use controller::Controller;
use termion_view::TermionView;
use gridsnakes::{Game, World};


fn main() {
    let size = 20;
    let snakes = 2;
    let mut controller = Controller::new(Game::new(World::new(size, size, snakes)), TermionView::new());
    controller.run_loop();
}
