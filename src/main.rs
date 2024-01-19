mod ui;

use ui::Error;

use std::time::Duration;
use tokio::runtime::Runtime;

fn main() -> Result<(), Error> {
    println!("Init async runtime");
    let rt = Runtime::new().expect("Failed to create async runtime");
    let _enter = rt.enter();
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
