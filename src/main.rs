mod ui;
mod world;
use ui::Error;
use world::world::World;

use rand::prelude::*;
use std::time::Duration;
use tokio::runtime::Runtime;

fn main() -> Result<(), Error> {
    println!("Init async runtime");
    let rt = Runtime::new().expect("Failed to create async runtime");
    let _enter = rt.enter();
    let mut rng = rand::thread_rng();
    let seed: u16 = rng.gen_range(1..100);
    let current_world = World::new(seed);

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });
    println!("Now in tokio runtime");

    println!("Render UI");
    println!("Current world spawns {:?}", current_world.spawns);
    // Create Spawner

    // Spawner creates tribes
    // Sends "Tribe Created" message
    // Create Tribe Supervisors for each tribe
    // Tribes start to fill themselves - tribes start with
    //     1 leader and 5-10 individuals (random)
    // Tribe Supervisor has code that runs on an interval (every ~10 seconds)
    //    (loop block with sleep(10) call)
    //    - Checks state of all individuals, updates tribe status
    //    - Based on status grows tribe (by sending a message to the Spawner)
    // Tribe Supervisor also has code that runs on a faster interval to move
    //     individuals
    //    (loop block with sleep(2) call)
    ui::main_screen(current_world)
}
