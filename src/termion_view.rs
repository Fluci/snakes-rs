use crate::controller::{View, UserAction};
use crate::model::*;

use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, Stdout};

pub struct TermionView {
    events: termion::input::Events<termion::AsyncReader>,
    stdout: termion::raw::RawTerminal<Stdout>
}

impl TermionView {
    pub fn new() -> TermionView {
        TermionView {
            events: termion::async_stdin().events(),
            stdout: stdout().into_raw_mode().unwrap()
        }
    }
}

impl View for TermionView {
    fn read_user_inputs(&mut self) -> Vec<UserAction> {
        let mut dirs: Vec<Option<Orientation>> = vec![None; 2];
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
                result.push(UserAction::Player(i, dirs[i].unwrap()));
            }
        }
        if quit {
            result.push(UserAction::Quit);
        }
        result
    }
    fn draw_world(&mut self, world: &World) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        
        for row in 0..world.grid.rows() {
            for col in 0..world.grid.cols() {
                write!(self.stdout, "{}", termion::cursor::Goto((col+1) as u16, (row+1) as u16)).unwrap();
                match world.grid.get((row, col)) {
                    Cell::Empty => write!(self.stdout, "_").unwrap(),
                    Cell::Food(_) => write!(self.stdout, "F").unwrap(),
                    Cell::Stone(_) => write!(self.stdout, "R").unwrap(),
                    Cell::Snake(s, _) => {
                        if world.is_head(*s, (row, col)) {write!(self.stdout, "o").unwrap();}
                        else if world.is_tail(*s, (row, col)) {write!(self.stdout, "-").unwrap();}
                        else if world.is_body(*s, (row, col)) {write!(self.stdout, "=").unwrap();}
                    }
                };
            }
        }
        
        self.stdout.flush().unwrap();
    }
}

