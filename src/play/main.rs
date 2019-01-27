extern crate gridsnakes;

use gridsnakes::view::{Controller, TermionView};
use gridsnakes::model::{Game, World};


fn main() {
    let size = 20;
    let snakes = 2;
    let mut controller = Controller::new(Game::new(World::new(size, size, snakes)), TermionView::new());
    controller.run_loop();
}
