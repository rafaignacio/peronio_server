use std::{sync::Arc, time::Duration};

use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    sync::{broadcast, Mutex},
};

use crate::world::world_event::WorldEvent;

#[derive(Default)]
pub struct Position(usize, usize);

pub struct Player {
    id: u64,
    connection: TcpStream,
    receiver: broadcast::Receiver<WorldEvent>,
    position: Position,
}

impl Player {
    pub fn spawn(id: u64, connection: TcpStream, receiver: broadcast::Receiver<WorldEvent>) {
        let player = Arc::new(Mutex::new(Player {
            id,
            connection,
            receiver,
            position: Default::default(),
        }));

        let cloned = player.clone();
        tokio::spawn(async move {
            let mut p = cloned.lock().await;
            let mut buf = [0; 1024];

            while let Ok(n) = p.connection.read(&mut buf).await {
                println!("message read from player {}: {:?}", p.id, &buf[..n]);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });

        let p2 = player.clone();
        tokio::spawn(async move {
            let mut r = p2.lock().await;
            println!("awaiting events");
            while let Ok(m) = r.receiver.recv().await {
                println!("event received {}", m);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }

            println!("exiting events thread");
        });
    }
}
