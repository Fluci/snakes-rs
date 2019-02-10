extern crate gridsnakes;
extern crate clap;

use gridsnakes::view::{Controller, TermionView};
use gridsnakes::model::{Game, World, Orientation};
use clap::{App, Arg};


fn main() {
    let matches = App::new("play snakes")
        .about("Snakes for one or two players.")
        .arg(Arg::with_name("multiplayer")
            .short("m")
            .help("Multiplayer?"))
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .value_name("GRID_SIZE")
            .help("Number of cells along each grid axis."))
        .arg(Arg::with_name("apples")
            .short("a")
            .long("apples")
            .value_name("APPLES")
            .help("Maximum number of apples at the same time in the world."))
        .arg(Arg::with_name("walls")
            .short("w")
            .long("walls")
            .help("Enable walls?"))
        .get_matches();
    let size = match matches.value_of("size") {Some(v) => match v.parse::<usize>() {Ok(n) => n, _ => 20}, _ => 20};
    let snakes = match matches.occurrences_of("multiplayer") {
        1 => 2,
        _ => 1
    };
    let snacks = match matches.value_of("apples") {Some(v) => match v.parse::<usize>() {Ok(n) => n, _ => (size*size/100+1)}, _ => (size*size/100 + 1)};
    let walls_enabled = match matches.occurrences_of("walls") {1 => true, _ => false};
    let mut controller = Controller::new(Game::new(World::new(size, size)), TermionView::new());
    for i in 0..snakes {
        controller.game.world.add_snake((1, (i*4+2)%size), Orientation::Down).unwrap();
    }
    controller.game.max_snacks = snacks;
    controller.game.world.wall_collision = walls_enabled;
    controller.run_loop();
}
