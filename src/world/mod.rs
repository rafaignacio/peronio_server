use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tokio::net::TcpListener;

use crate::player::Player;

const SERVER_PORT: u16 = 8555;

#[derive(Default)]
pub struct World {
    players: Arc<Mutex<Vec<Player>>>,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn run(&self) {
        //TODO: (de)spawn players
        World::spawn_players(self.players.clone());
        //TODO: spawn creatures spawner
        //TODO: update events

        loop {
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn spawn_players(players: Arc<Mutex<Vec<Player>>>) {
        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", SERVER_PORT))
                .await
                .unwrap();
            println!("Listening on port {}...", SERVER_PORT);

            loop {
                let (socket, _) = listener.accept().await.unwrap();
                println!("User connected");
                let mut players = players.lock().unwrap();

                players.push(Player::new(socket));
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
}
