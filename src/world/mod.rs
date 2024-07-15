pub mod world_event;

use crate::player::Player;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
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
            players: Default::default(),
        }
    }

    pub async fn run(&self) {
        println!("Initiating world..");
        self.spawn_players();
        //TODO: spawn creatures spawner
        //TODO: update events

        loop {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    fn spawn_players(&self) {
        let players = self.players.clone();
        let cloned = self.producer.clone();

        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT))
                .await
                .unwrap();
            println!("Listening on port {}...", SERVER_PORT);
            let mut user_id: u64 = 1;

            loop {
                println!("awaiting user connection..");
                let (socket, _) = listener.accept().await.unwrap();
                println!("user connected");
                let mut players = players.lock().unwrap();

                players.push(Player::new(user_id, socket, cloned.subscribe()));
                user_id += 1;
            }
        });
    }
}
