use std::{io::Error, time::Duration};

use tokio::net::TcpStream;

use crate::world::World;

const SERVER_PORT: u16 = 8555;

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn should_run_game() {
    // Inicia o mundo do jogo em uma tarefa separada
    let world = tokio::spawn(async {
        World::new().run().await;
    });

    // Aguarda o servidor iniciar
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Conecta o primeiro jogador
    let player1_conn = connect_player().await;
    let player1 = match player1_conn {
        Ok(conn) => conn,
        Err(e) => {
            world.abort();
            drop(world);
            panic!("Failed to connect player 1: {}", e);
        }
    };

    // Escreve uma mensagem a partir do jogador 1
    if let Err(e) = player1.try_write(b"Hello world!") {
        eprintln!("Failed to write message from player 1: {}", e);
        world.abort();
        drop(world);
        panic!("Failed to write with player 1: {}", e);
    } else {
        println!("Player 1 wrote a message successfully.");
    }

    // Conecta o segundo jogador
    let player2_conn = connect_player().await;
    let player2 = match player2_conn {
        Ok(conn) => conn,
        Err(e) => {
            world.abort();
            drop(world);
            panic!("Failed to connect player 2: {}", e);
        }
    };

    // Escreve uma mensagem a partir do jogador 2
    if let Err(e) = player2.try_write(b"Hello world from player 2!") {
        eprintln!("Failed to write message from player 2: {}", e);
        world.abort();
        drop(world);
        panic!("Failed to write with player 2: {}", e);
    } else {
        println!("Player 2 wrote a message successfully.");
    }

    // Encerra o mundo do jogo
    world.abort();
}

async fn connect_player() -> Result<TcpStream, Error> {
    TcpStream::connect(format!("127.0.0.1:{}", SERVER_PORT)).await
}
