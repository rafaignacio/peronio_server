mod player;
mod world;

use std::fmt::Error;
use world::World;

#[tokio::main]
async fn main() -> Result<(), Error> {
    World::new().run().await;

    Ok(())
}
