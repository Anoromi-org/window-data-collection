// This runs daemon on windows without creating a console. Disable during development to see
// stdout.

use std::{path::Path, thread::sleep, time::Duration};

use anyhow::Result;
use tracing::{error, info, level_filters::LevelFilter};
use whatawhat::{utils::logging::enable_logging, window_api::{GenericWindowManager, WindowManager}};

#[tokio::main]
async fn main() -> Result<()> {
    enable_logging("example", "logs".as_ref(), Some(LevelFilter::TRACE), true)?;
    let mut manager = GenericWindowManager::new().await.inspect_err(|e| error!("Failed to create window manager {e:?}"))?;
    loop {
        info!("Window data {:?}", manager.get_active_window_data().await);
        info!("Idle time{:?}", manager.get_idle_time().await);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
