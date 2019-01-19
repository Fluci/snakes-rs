extern crate termion;

mod controller;
mod model;
mod termion_view;

use controller::{Controller};
use termion_view::TermionView;


fn main() {
    let size = 10;
    let mut controller = Controller::with_size(size, size, TermionView::new());
    controller.run_loop();
}
