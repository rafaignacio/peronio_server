use tokio::{net::TcpStream, sync::broadcast};

use crate::world::world_event::WorldEvent;

pub struct Player {
    id: u64,
    connection: TcpStream,
    receiver: broadcast::Receiver<WorldEvent>,
}

impl Player {
    pub fn new(id: u64, connection: TcpStream, receiver: broadcast::Receiver<WorldEvent>) -> Self {
        Player {
            id,
            connection,
            receiver,
        }
    }
}
