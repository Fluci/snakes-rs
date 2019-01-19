use std::{thread, time};
use crate::model::{World, Orientation};


pub trait View {
    fn read_user_inputs(&mut self) -> Vec<UserAction>;
    fn draw_world(&mut self, world: &World);
}

pub enum UserAction {
    Quit,
    Player(i32, Orientation)
}

pub struct Controller<V: View + Sized> {
    world: World,
    view: V,
    iteration: usize
}


impl<V: View + Sized> Controller<V> {
    pub fn with_size(rows: usize, cols: usize, view: V) -> Controller<V> {
        Controller {
            world: World::with_size(rows, cols),
            view: view,
            iteration: 0
        }
    }
    pub fn run_loop(&mut self){
        self.view.read_user_inputs(); // drop any user input
        self.view.draw_world(&self.world);
        thread::sleep(time::Duration::from_millis(1000/1));
        loop{
            let mut direction = self.world.snake_direction(self.world.snakes[0].head);
            let actions = self.view.read_user_inputs();
            if self.iteration % 8 == 0 {
                self.world.place_snack((4,4), 1);
            }
            for a in actions {
                match a {
                    UserAction::Quit => return,
                    UserAction::Player(_pid, dir) => direction = dir
                }
            }
            // Physics
            self.world.advance(direction);
            // Display on screen
            self.view.draw_world(&self.world);
            thread::sleep(time::Duration::from_millis(1000/2));
            self.iteration += 1;
        } 
    }
}

