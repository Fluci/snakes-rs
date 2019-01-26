extern crate termion;
extern crate rand;

mod controller;
mod model;
mod termion_view;

use controller::Controller;
use termion_view::TermionView;
use model::{Game, World};


fn main() {
    let size = 20;
    let snakes = 2;
    let mut controller = Controller::new(Game::new(World::new(size, size, snakes)), TermionView::new());
    controller.run_loop();
}
