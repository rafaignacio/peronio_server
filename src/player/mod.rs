pub mod spawner;

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    id: u64,
}

impl Player {
    pub fn new(id: u64) -> Self {
        Player { id }
    }
}
