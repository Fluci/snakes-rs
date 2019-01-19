use std::{thread, time};
use crate::model::{World, Orientation};


pub trait View {
    fn read_user_inputs(&mut self) -> Vec<UserAction>;
    fn draw_world(&mut self, world: &World);
}

pub enum UserAction {
    Quit,
    Player(usize, Orientation)
}

pub struct Controller<V: View + Sized> {
    world: World,
    view: V,
    iteration: usize
}


impl<V: View + Sized> Controller<V> {
    pub fn new(world: World, view: V) -> Controller<V> {
        Controller {
            world: world,
            view: view,
            iteration: 0
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
            if self.iteration % 8 == 0 {
                self.world.place_snack((4,4), 1);
            }
            for a in actions {
                match a {
                    UserAction::Quit => return,
                    UserAction::Player(pid, dir) => directions[pid] = dir
                }
            }
            // Physics
            self.world.advance(&directions);
            // Display on screen
            self.view.draw_world(&self.world);
            thread::sleep(time::Duration::from_millis(1000/2));
            self.iteration += 1;
        } 
    }
}

