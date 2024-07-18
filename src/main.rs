use std::io::Error;

use world::World;

pub mod player;
pub mod world;

#[tokio::main]
async fn main() -> Result<(), Error> {
    World::new().run().await
}
