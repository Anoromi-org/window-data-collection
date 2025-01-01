// This is an example to test activity data on different operating systems.

pub mod utils;

use std::time::Duration;

use anyhow::Result;
use tracing::{ error, info, level_filters::LevelFilter };
use window_data_collection::window_api::{ ActiveWindowManager, GenericActiveWindowManager };
use utils::logging::enable_logging;


#[ tokio::main ]
async fn main() -> Result< () >
{
  enable_logging( "example", "logs".as_ref(), Some( LevelFilter::TRACE ), true )?;
  let mut manager = GenericActiveWindowManager::new()
  .await
  .inspect_err( | e | error!( "Failed to create window manager {e:?}" ) )?;

  info!( "Created manager" );
  loop
  {
    info!( "Window data {:?}", manager.get_active_window_data().await );
    tokio::time::sleep( Duration::from_secs( 1 ) ).await;
  }
}
