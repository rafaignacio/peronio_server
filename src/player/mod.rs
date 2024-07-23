pub mod spawner;

use crate::world::Action;

#[derive(Debug, Clone, Copy)]
pub struct Position(u64, u64);

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    id: u64,
    position: Option<Position>,
}

impl Player {
    pub fn new(id: u64) -> Self {
        Player {
            id,
            ..Default::default()
        }
    }

    pub fn do_action(&mut self, action: Action) {
        match action {
            Action::Move(x, y) => (),
        }
    }
}
