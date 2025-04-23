// This is an example to test activity data on different operating systems.

pub mod utils;

use std::{thread::sleep, time::Duration};

use anyhow::Result;
use tracing::{ error, info, level_filters::LevelFilter };
use window_data_collecting::window_api::{ ActiveWindowManager, GenericActiveWindowManager };
use utils::logging::enable_logging;


fn main() -> Result< () >
{
  enable_logging( "example", "logs".as_ref(), Some( LevelFilter::TRACE ), true )?;
  let mut manager = GenericActiveWindowManager::new()
  .inspect_err( | e | error!( "Failed to create window manager {e:?}" ) )?;

  info!( "Created manager" );
  loop
  {
    info!( "Window data {:?}", manager.get_active_window_data() );
    sleep( Duration::from_secs( 1 ) );
  }
}
