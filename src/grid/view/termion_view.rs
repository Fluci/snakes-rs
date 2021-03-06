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
    pub fn new() -> Result<TermionView, ()> {
        let std = match stdout().into_raw_mode() {
            Ok(v) => v,
            Err(_) => return Err(())
        };
        Ok(TermionView {
            events: termion::async_stdin().events(),
            stdout: std,
            players: 2
        })
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
    fn game_running(&mut self, world: &World) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        
        for row in 0..world.grid.rows() {
            for col in 0..world.grid.cols() {
                write!(self.stdout, "{}", termion::cursor::Goto((col+1) as u16, (row+1) as u16)).unwrap();
                match world.grid.get((row, col)) {
                    Cell::Empty => write!(self.stdout, "_").unwrap(),
                    Cell::Food(1) => write!(self.stdout, "'").unwrap(),
                    Cell::Food(2) => write!(self.stdout, "^").unwrap(),
                    Cell::Food(_) => write!(self.stdout, "A").unwrap(),
                    Cell::Stone => write!(self.stdout, "!").unwrap(),
                    Cell::Snake(s, d) => {
                        self.player_color(*s);
                        if world.is_head(*s, (row, col)) {write!(self.stdout, "o").unwrap();}
                        else if world.is_tail(*s, (row, col)) {write!(self.stdout, ".").unwrap();}
                        else if world.is_body(*s, (row, col)) && (*d == Orientation::Up || *d == Orientation::Down) {write!(self.stdout, "|").unwrap();}
                        else if world.is_body(*s, (row, col)) && (*d == Orientation::Left || *d == Orientation::Right) {write!(self.stdout, "=").unwrap();}
                        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
                    }
                };
            }
        }

        write!(self.stdout, "{}", termion::cursor::Goto(1, 1 + world.grid.rows() as u16 )).unwrap();
    }
    fn game_over(&mut self, world: &World) {
        self.game_running(world);
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1 + world.grid.rows() as u16 )).unwrap();
        if world.snakes.len() > 1 {
            self.player_color(world.winners[0]);
            write!(self.stdout, "Player {} wins!{}\n\r", 
                world.winners[0]+1, 
                color::Fg(color::Reset)
            ).unwrap();
        } else {
            self.player_color(0);
            if world.winners.is_empty() {
                write!(self.stdout, "Loss! Reached length: {}\n\r{}", world.snakes[0].length, color::Fg(color::Reset)).unwrap();
            } else {
                write!(self.stdout, "Win! Reached length: {}\n\r{}", world.snakes[0].length, color::Fg(color::Reset)).unwrap();
            }
        }
    }
    fn game_draw(&mut self, world: &World) {
        self.game_running(world);
        write!(self.stdout, "{}Draw!\n\r", termion::cursor::Goto(1, 1 + world.grid.rows() as u16)).unwrap();
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
    fn draw_world(&mut self, world: &World) {
        match world.turn_result {
            TurnResult::Ok => self.game_running(world),
            TurnResult::GameOver => self.game_over(world),
            TurnResult::Draw => self.game_draw(world)
        }
        self.stdout.flush().unwrap();
    }
}

