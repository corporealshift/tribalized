mod ui;
mod world;
use ui::Error;
use world::world::World;

use std::time::Duration;
use tokio::runtime::Runtime;

fn main() -> Result<(), Error> {
    println!("Init async runtime");
    let rt = Runtime::new().expect("Failed to create async runtime");
    let _enter = rt.enter();
    let current_world = World { map: vec![] };

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });
    println!("Now in tokio runtime");

    println!("Render UI");
    ui::main_screen()
}
