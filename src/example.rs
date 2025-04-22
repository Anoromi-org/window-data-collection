// This runs daemon on windows without creating a console. Disable during development to see
// stdout.

use std::{path::Path, thread::sleep, time::Duration};

use anyhow::Result;
use tracing::level_filters::LevelFilter;
use whatawhat::{utils::logging::enable_logging, window_api::{GenericWindowManager, WindowManager}};

#[tokio::main]
async fn main() -> Result<()> {
    enable_logging("example", "logs".as_ref(), Some(LevelFilter::TRACE), true)?;
    let mut manager = GenericWindowManager::new().await?;
    loop {
        println!("Window data {:?}", manager.get_active_window_data().await);
        println!("Idle time{:?}", manager.get_idle_time().await);
        println!();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
