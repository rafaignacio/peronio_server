pub(crate) struct PlayerSpawner;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{
        broadcast::{Receiver, Sender},
        mpsc, Mutex,
    },
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
                    let action_receiver =
                        Arc::new(Mutex::new(action_sender.lock().await.subscribe()));
                    spawn_players(l, players, action_receiver, command_sender).await
                }
                Err(e) => {
                    eprintln!("Failed to open up server on port {PORT}.\r\nErr: {e:?}");
                }
            }
        });
    }
}

async fn spawn_players(
    listener: TcpListener,
    players: Arc<Mutex<HashMap<u64, Player>>>,
    action_receiver: Arc<Mutex<Receiver<Action>>>,
    command_sender: Arc<Mutex<mpsc::UnboundedSender<Command>>>,
) {
    loop {
        println!("Awaiting user connection");
        let socket = listener.accept().await;

        println!("User connected");
        match socket {
            Ok((stream, _)) => {
                let spawned_player = spawn_player(&players).await;
                let player = spawned_player.lock().await;
                let connection = Arc::new(Mutex::new(stream));
                let (write_tx, write_rx) = mpsc::unbounded_channel();

                tokio::spawn(handle_player_communication(
                    player.id,
                    Arc::clone(&connection),
                    write_rx,
                    command_sender.clone(),
                ));

                listen_player_commands(
                    player.id,
                    Arc::clone(&connection),
                    &command_sender,
                    write_tx.clone(),
                );
                listen_world_actions(
                    Arc::clone(&spawned_player),
                    Arc::clone(&action_receiver),
                    write_tx,
                );
            }
            Err(e) => eprintln!("Failed to accept user connection.\r\nError: {e:?}"),
        }
    }
}

fn listen_world_actions(
    player: Arc<Mutex<Player>>,
    action_receiver: Arc<Mutex<Receiver<Action>>>,
    write_tx: mpsc::UnboundedSender<Vec<u8>>,
) {
    tokio::spawn(async move {
        let mut player = player.lock().await;
        let mut receiver = action_receiver.lock().await;

        loop {
            match receiver.recv().await {
                Ok(action) => {
                    player.do_action(action);
                    if let Err(e) = write_tx.send(b"TEST RESPONSE".to_vec()) {
                        eprintln!(
                            "Failed to send response to user {}. \r\nErr: {e:?}",
                            player.id
                        );
                    }
                }
                Err(e) => eprintln!("Failed while listening to world actions.\r\nErr: {e:?}"),
            }
        }
    });
}

fn listen_player_commands(
    player_id: u64,
    stream: Arc<Mutex<TcpStream>>,
    command_sender: &Arc<Mutex<mpsc::UnboundedSender<Command>>>,
    write_tx: mpsc::UnboundedSender<Vec<u8>>,
) {
    let command_sender = Arc::clone(command_sender);

    tokio::spawn(async move {
        let mut buf: [u8; 1024] = [0; 1024];
        let mut stream = stream.lock().await;

        while let Ok(n) = stream.read(&mut buf).await {
            if n == 0 {
                break;
            }
            if let Ok(msg) = std::str::from_utf8(&buf[..n]) {
                println!("Message received from player {player_id}: {msg}");
                // Process the command and send responses back using write_tx if needed.
                // write_tx.send(b"Some response".to_vec()).unwrap();
            }
        }

        let _ = command_sender
            .lock()
            .await
            .send(Command::UserDisconnected(player_id));
    });
}

async fn handle_player_communication(
    player_id: u64,
    stream: Arc<Mutex<TcpStream>>,
    mut write_rx: mpsc::UnboundedReceiver<Vec<u8>>,
    command_sender: Arc<Mutex<mpsc::UnboundedSender<Command>>>,
) {
    tokio::spawn(async move {
        let mut stream = stream.lock().await;

        while let Some(msg) = write_rx.recv().await {
            if let Err(e) = stream.write(&msg).await {
                eprintln!("Failed to write back to user {}. \r\nErr: {e:?}", player_id);
                let _ = command_sender
                    .lock()
                    .await
                    .send(Command::UserDisconnected(player_id));
                break;
            }
        }
    });
}

async fn spawn_player(players: &Arc<Mutex<HashMap<u64, Player>>>) -> Arc<Mutex<Player>> {
    let mut players = players.lock().await;
    let id = (players.len() as u64) + 1;
    let player = Player::new(id);

    players.insert(id, player);
    Arc::new(Mutex::new(player))
}
