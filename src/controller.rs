use std::{thread, time};
use crate::model::{World, Orientation, GameEvent};


pub trait View {
    fn read_user_inputs(&mut self) -> Vec<UserAction>;
    fn draw_world(&mut self, world: &World);
    fn player_wins(&mut self, player: usize, world: &World);
    fn draw(&mut self, wolrd: &World);
}

pub enum UserAction {
    Quit,
    Player(usize, Orientation)
}

pub struct Controller<V: View + Sized> {
    pub world: World,
    pub view: V,
    iteration: usize,
    pub quit_on_collision: bool
}


impl<V: View + Sized> Controller<V> {
    pub fn new(world: World, view: V) -> Controller<V> {
        Controller {
            world: world,
            view: view,
            iteration: 0,
            quit_on_collision: true
        }
    }
    pub fn run_loop(&mut self){
        self.view.read_user_inputs(); // drop any user input
        self.view.draw_world(&self.world);
        thread::sleep(time::Duration::from_millis(1000/1));
        let mut directions = vec![Orientation::Down; self.world.player_count()];
        loop{
            for i in 0..self.world.player_count() {
                directions[i] = self.world.snake_direction(self.world.snakes[i].head);
            }
            let actions = self.view.read_user_inputs();
            if self.iteration % 16 == 0 && self.world.available_snacks() < 2 {
                self.world.place_snack_randomly((2*self.iteration)%3+1).unwrap();
            }
            for a in actions {
                match a {
                    UserAction::Quit => return,
                    UserAction::Player(pid, dir) => directions[pid] = dir
                }
            }
            // Physics
            let events = self.world.advance(&directions);
            // Display on screen
            self.view.draw_world(&self.world);
            let mut players_collided = vec![false; self.world.player_count()];
            for e in events {
                match e {
                    GameEvent::Collision(s, _) => players_collided[s] = true,
                    _ => ()
                };
            }
            let all_collided = players_collided.iter().fold(true, |sum, x| sum && *x);
            let some_collided = players_collided.iter().fold(false, |sum, x| sum || *x);
            if all_collided {
                self.view.draw(&self.world);
                if self.quit_on_collision {
                    return;
                }
            }
            else if some_collided {
                // TODO: only for exactly two players
                if players_collided[0] {
                    self.view.player_wins(1, &self.world);
                } else {
                    self.view.player_wins(0, &self.world);
                }
                if self.quit_on_collision {
                    return;
                }
            }
            thread::sleep(time::Duration::from_millis(1000/2));
            self.iteration += 1;
        } 
    }
}

