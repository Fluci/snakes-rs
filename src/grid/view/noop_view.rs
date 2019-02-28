use crate::model::*;

use super::controller::{View, UserAction};

pub struct NoopView {}

/// View that just throws away any command. For headless mode.
impl NoopView {
    pub fn new() -> NoopView {
        NoopView{}
    }
}

impl View for NoopView {
    fn read_user_inputs(&mut self) -> Vec<UserAction> {
        Vec::new()
    }
    fn draw_world(&mut self, world: &World) {}
}

