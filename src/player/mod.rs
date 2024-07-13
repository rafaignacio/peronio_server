use std::{thread, time::Duration};

use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct Player;

impl Player {
    pub fn new(mut connection: TcpStream) -> Self {
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let _ = match connection.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprint!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                thread::sleep(Duration::from_millis(10));
            }
        });
        Player {}
    }
}
