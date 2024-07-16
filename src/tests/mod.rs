use std::{io::Error, time::Duration};

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::world::World;

const SERVER_PORT: u16 = 8555;

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn should_run_game() {
    let world = tokio::spawn(async {
        World::new().run().await;
    });

    //gives time to server to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    let player1_conn = connect_player().await;

    let Ok(mut player1) = player1_conn else {
        world.abort();
        drop(world);
        panic!("failed: {}", player1_conn.unwrap_err());
    };
    // Write some data.
    match player1.try_write(b"Hello world!") {
        Ok(n) => {
            println!("total bytes wrote:{n}");
        }
        Err(e) => {
            eprintln!("failed to write message: {}", e);
            panic!("failed with {e}");
        }
    };

    let player2_conn = connect_player().await;
    let Ok(player2) = player2_conn else {
        world.abort();
        drop(world);
        panic!("failed: {}", player2_conn.unwrap_err());
    };

    match player2.try_write(b"Hello world from player 2!") {
        Ok(n) => {
            println!("total bytes wrote:{n}");
        }
        Err(e) => {
            eprintln!("failed to write message: {}", e);
            panic!("failed with {e}");
        }
    };

    world.abort();
}

async fn connect_player() -> Result<TcpStream, Error> {
    TcpStream::connect(format!("127.0.0.1:{}", SERVER_PORT)).await
}
