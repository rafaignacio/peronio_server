pub mod world_event;

use crate::player::Player;
use async_std::sync::Mutex;
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpListener, sync::broadcast};
use world_event::WorldEvent;

const SERVER_PORT: u16 = 8555;

#[derive(Clone)]
pub struct World {
    producer: Arc<tokio::sync::broadcast::Sender<WorldEvent>>,
    players: Arc<Mutex<Vec<Player>>>,
}

impl World {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(2048);
        World {
            producer: Arc::new(tx),
            players: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn run(&self) {
        println!("Initiating world..");
        self.spawn_players();

        loop {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    fn spawn_players(&self) {
        let cloned = Arc::clone(&self.producer);
        let players = Arc::clone(&self.players);

        tokio::spawn(async move {
            match TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT)).await {
                Ok(listener) => {
                    println!("Listening on port {}...", SERVER_PORT);
                    let mut user_id: u64 = 1;

                    loop {
                        match listener.accept().await {
                            Ok((socket, _)) => {
                                println!("User connected with ID: {}", user_id);

                                let mut player = Player::new(user_id);
                                player.set_connection(socket);
                                player.set_receiver(cloned.subscribe());

                                player.spawn();
                                players.lock().await.push(player);
                                user_id += 1;
                            }
                            Err(e) => {
                                eprintln!("Failed to accept connection: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to bind to port {}: {:?}", SERVER_PORT, e);
                }
            }
        });
    }
}
