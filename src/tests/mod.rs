use std::time::Duration;

use tokio::net::TcpStream;

use crate::world::World;

const SERVER_PORT: u16 = 8555;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_run_game() {
    let world = tokio::spawn(async {
        World::new().run().await;
    });

    //gives time to server to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    let connect = TcpStream::connect(format!("127.0.0.1:{}", SERVER_PORT)).await;
    let Ok(stream) = connect else {
        world.abort();
        drop(world);
        panic!("failed: {}", connect.unwrap_err());
    };
    // Write some data.
    match stream.try_write(b"Hello world!") {
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
