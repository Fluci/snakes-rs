extern crate termion;

mod controller;
mod model;
mod termion_view;

use controller::Controller;
use termion_view::TermionView;
use model::World;


fn main() {
    let size = 10;
    let snakes = 2;
    let mut controller = Controller::new(World::new(size, size, snakes), TermionView::new());
    controller.run_loop();
}
