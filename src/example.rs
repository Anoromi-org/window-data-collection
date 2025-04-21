// This runs daemon on windows without creating a console. Disable during development to see
// stdout.

use std::{thread::sleep, time::Duration};

use anyhow::Result;
use whatawhat::window_api::{GenericWindowManager, WindowManager};

#[tokio::main]
async fn main() -> Result<()> {
    let mut manager = GenericWindowManager::new().await?;
    loop {
        println!("Window data {:?}", manager.get_active_window_data().await);
        println!("Idle time{:?}", manager.get_idle_time().await);
        println!();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
