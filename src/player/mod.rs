use std::{sync::Arc, time::Duration};

use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    sync::{broadcast::Receiver, Mutex},
};

use crate::world::world_event::WorldEvent;

#[derive(Debug, Default)]
pub struct Position(usize, usize);

#[derive(Debug, Default)]
pub struct Player {
    id: u64,
    connection: Option<Arc<Mutex<TcpStream>>>,
    receiver: Option<Arc<Mutex<Receiver<WorldEvent>>>>,
    position: Option<Position>,
}

impl Player {
    pub fn new(id: u64) -> Self {
        Player {
            id,
            ..Default::default()
        }
    }

    pub fn set_connection(&mut self, connection: TcpStream) {
        self.connection = Some(Arc::new(Mutex::new(connection)));
    }

    pub fn set_receiver(&mut self, receiver: Receiver<WorldEvent>) {
        self.receiver = Some(Arc::new(Mutex::new(receiver)));
    }

    pub fn spawn(&self) {
        if let Some(conn) = &self.connection {
            let conn = Arc::clone(conn);
            let id = self.id;
            tokio::spawn(async move {
                Self::listen_connection_commands(conn, id).await;
            });
        }

        if let Some(receiver) = &self.receiver {
            let rcv = Arc::clone(receiver);
            tokio::spawn(async move {
                Self::listen_world_events(rcv).await;
            });
        }
    }

    async fn listen_connection_commands(conn: Arc<Mutex<TcpStream>>, id: u64) {
        let mut buf = [0; 1024];

        loop {
            let n = {
                let mut conn_lock = conn.lock().await;
                match conn_lock.read(&mut buf).await {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Error reading from connection for player {}: {:?}", id, e);
                        return;
                    }
                }
            };

            if n == 0 {
                println!("Connection closed for player {}", id);
                return;
            }

            println!("Message read from player {}: {:?}", id, &buf[..n]);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    async fn listen_world_events(receiver: Arc<Mutex<Receiver<WorldEvent>>>) {
        println!("awaiting events");
        while let Ok(m) = receiver.lock().await.recv().await {
            println!("event received {}", m);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        println!("exiting events thread");
    }
}
