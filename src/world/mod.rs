use std::{collections::HashMap, fmt::Display, io::Error, sync::Arc};

use tokio::sync::{
    broadcast::{self, Sender},
    mpsc::{self, UnboundedReceiver},
    Mutex,
};

use crate::player::{spawner::PlayerSpawner, Player};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Move(u64, u64),
}
#[derive(Debug, Clone, Copy)]
pub enum Command {
    UserDisconnected(u64),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Command::UserDisconnected(id) => write!(f, "User {id} disconnected."),
        }
    }
}

#[derive(Debug)]
pub struct World {
    action_sender: Arc<Mutex<Sender<Action>>>,
    command_sender: Arc<Mutex<mpsc::UnboundedSender<Command>>>,
    command_receiver: Arc<Mutex<UnboundedReceiver<Command>>>,
    players: Arc<Mutex<HashMap<u64, Player>>>,
}

impl Default for World {
    fn default() -> Self {
        let (action_sender, _) = broadcast::channel::<Action>(2048);
        let (command_sender, command_receiver) = mpsc::unbounded_channel();
        World {
            action_sender: Arc::new(Mutex::new(action_sender)),
            command_sender: Arc::new(Mutex::new(command_sender)),
            command_receiver: Arc::new(Mutex::new(command_receiver)),
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn run(&self) -> Result<(), Error> {
        println!("Initiating world");
        let (players, action_sender, command_sender) = (
            Arc::clone(&self.players),
            Arc::clone(&self.action_sender),
            Arc::clone(&self.command_sender),
        );

        PlayerSpawner::run(players, action_sender, command_sender);
        let mut receiver = self.command_receiver.lock().await;
        while let Some(msg) = receiver.recv().await {
            println!("Command received: {}", msg);
        }

        Ok(())
    }
}
