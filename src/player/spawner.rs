pub(crate) struct PlayerSpawner;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{broadcast::Sender, mpsc, Mutex},
};

use super::Player;
use crate::world::{Action, Command};

const PORT: u16 = 8555;

impl PlayerSpawner {
    pub(crate) fn run(
        players: Arc<Mutex<HashMap<u64, Player>>>,
        action_sender: Arc<Mutex<Sender<Action>>>,
        command_sender: Arc<Mutex<mpsc::UnboundedSender<Command>>>,
    ) {
        println!("Initiating Player Spawner thread");
        tokio::spawn(async move {
            let listener = TcpListener::bind(format!("127.0.0.1:{PORT}")).await;

            match listener {
                Ok(l) => {
                    println!("Listening on 127.0.0.1:{PORT}");
                    spawn_players(l, players).await
                }
                Err(e) => {
                    eprintln!("Failed to open up server on port {PORT}.\r\nErr: {e:?}");
                }
            }
        });
    }
}

async fn spawn_players(listener: TcpListener, players: Arc<Mutex<HashMap<u64, Player>>>) {
    loop {
        let socket = listener.accept().await;
        match socket {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to accept user connection.\r\nError: {e:?}"),
        }
    }
}
