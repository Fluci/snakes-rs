use crate::model::*;

use super::controller::{View, UserAction};

use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color};
use std::io::{Write, stdout, Stdout};

pub struct TermionView {
    events: termion::input::Events<termion::AsyncReader>,
    stdout: termion::raw::RawTerminal<Stdout>,
    players: usize
}

impl TermionView {
    pub fn new() -> TermionView {
        TermionView {
            events: termion::async_stdin().events(),
            stdout: stdout().into_raw_mode().unwrap(),
            players: 2
        }
    }
}

impl TermionView {
    fn player_color(&mut self, player: usize) {
        match player {
            0 => write!(self.stdout, "{}", color::Fg(color::Blue)).unwrap(),
            1 => write!(self.stdout, "{}", color::Fg(color::Green)).unwrap(),
            _ => write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap()
        }
    }
}

impl View for TermionView {
    fn read_user_inputs(&mut self) -> Vec<UserAction> {
        let mut dirs: Vec<Option<Orientation>> = vec![None; self.players];
        let mut quit = false;
        // User input
        loop {
            let event = self.events.next();
            match event {
                None => break,
                Some(Ok(Event::Key(e))) => match e {
                    Key::Left => dirs[0] = Some(Orientation::Left),
                    Key::Right => dirs[0] = Some(Orientation::Right),
                    Key::Up => dirs[0] = Some(Orientation::Up),
                    Key::Down => dirs[0] = Some(Orientation::Down),
                    Key::Char('a') => dirs[1] = Some(Orientation::Left),
                    Key::Char('d') => dirs[1] = Some(Orientation::Right),
                    Key::Char('w') => dirs[1] = Some(Orientation::Up),
                    Key::Char('s') => dirs[1] = Some(Orientation::Down),
                    Key::Char('q') => quit = true,
                    Key::Char(_) => (),
                    _ => (),
                },
                _ => ()
            };
        };
        let mut result = Vec::new();
        for i in 0..dirs.len() {
            if dirs[i].is_some() {
                result.push(UserAction::Player(i, PlayerInput::Go(dirs[i].unwrap())));
            }
        }
        if quit {
            result.push(UserAction::Quit);
        }
        result
    }
    fn player_wins(&mut self, winners: Vec<usize>, world: &World) {
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1 + world.grid.rows() as u16 )).unwrap();
        self.player_color(winners[0]);
        write!(self.stdout, "Player {} wins!{}\n\r", 
            winners[0]+1, 
            color::Fg(color::Reset)
        ).unwrap();
    }
    fn draw(&mut self, world: &World) {
        write!(self.stdout, "{}Draw!\n\r", termion::cursor::Goto(1, 1 + world.grid.rows() as u16)).unwrap();
    }
    fn draw_world(&mut self, world: &World) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        
        for row in 0..world.grid.rows() {
            for col in 0..world.grid.cols() {
                write!(self.stdout, "{}", termion::cursor::Goto((col+1) as u16, (row+1) as u16)).unwrap();
                match world.grid.get((row, col)) {
                    Cell::Empty => write!(self.stdout, "_").unwrap(),
                    Cell::Food(1) => write!(self.stdout, "'").unwrap(),
                    Cell::Food(2) => write!(self.stdout, "^").unwrap(),
                    Cell::Food(_) => write!(self.stdout, "A").unwrap(),
                    Cell::Stone(_) => write!(self.stdout, "!").unwrap(),
                    Cell::Snake(s, _) => {
                        self.player_color(*s);
                        if world.is_head(*s, (row, col)) {write!(self.stdout, "o").unwrap();}
                        else if world.is_tail(*s, (row, col)) {write!(self.stdout, ".").unwrap();}
                        else if world.is_body(*s, (row, col)) {write!(self.stdout, "=").unwrap();}
                        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
                    }
                };
            }
        }

        write!(self.stdout, "{}", termion::cursor::Goto( 1, 1 + world.grid.rows() as u16 )).unwrap();
        self.stdout.flush().unwrap();
    }
}

