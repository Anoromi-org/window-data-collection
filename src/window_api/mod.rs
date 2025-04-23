//! Contains logic for extracting records from different environments.
//! [GenericWindowManager] is the main artifact of this module that abstracts
//! the operations.

#[cfg(feature = "ext_wlnd")]
pub mod ext_list_wayland;
#[cfg(feature = "gnome_wlnd")]
pub mod gnome_wayland;
#[cfg(feature = "kde_wlnd")]
pub mod kde_wayland;
#[cfg(feature = "macos")]
pub mod macos;
pub mod process_name;
#[cfg(feature = "win")]
pub mod win;
#[cfg(any(feature = "wlr_wlnd", feature = "ext_wlnd"))]
pub mod wl_connection;
#[cfg(feature = "wlr_wlnd")]
pub mod wlr_management_wayland;
#[cfg(feature = "x11")]
pub mod x11;

#[cfg(feature = "win")]
extern crate windows;

#[cfg(feature = "x11")]
extern crate xcb;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWindowData {
  /// Name of the window. For example, 'bash in hello' or 'Document 1' or '*Video* in YouTube - Chrome'
  pub window_title: Arc<str>,
  /// Full path to an executable. For example, /home/etc/nvim
  pub process_name: Option<Arc<str>>,
  /// App id of the application. For example, firefox
  pub app_id: Option<Arc<str>>,
}

/// Intended to serve as a contract windows and linux systems must implement.
#[async_trait]
pub trait WindowManager: ActiveWindowManager {
  /// Retrieve amount of time user has been inactive in milliseconds
  async fn get_idle_time(&mut self) -> Result<u32>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ActiveWindowManager {
  async fn get_active_window_data(&mut self) -> Result<ActiveWindowData>;
}

#[cfg(test)]
mockall::mock! {
    // Structure to mock
  MockWindowManager {}
  // First trait to implement on C
  #[async_trait]
  impl ActiveWindowManager for MockWindowManager {
      async fn get_active_window_data(&mut self) -> Result<ActiveWindowData>;
  }
  // Second trait to implement on C
  #[async_trait]
  impl WindowManager for MockWindowManager {
      async fn get_idle_time(&mut self) -> Result<u32>;
  }
}

macro_rules! try_create_manager {
  ($call: expr) => {{
    let data = $call;
    match data {
      Ok(v) => return Ok(Self { inner: Box::new(v) }),
      Err(e) => Err(e) as Result<()>,
    }
  }};
}

/// Serves as a cross-compatible WindowManager implementation.
pub struct GenericActiveWindowManager {
  inner: Box<dyn ActiveWindowManager + Send>,
}

impl GenericActiveWindowManager {
  pub async fn new() -> Result<Self> {
    #[cfg(feature = "win")]
    {
      use win::WindowsWindowManager;
      return Ok(Self {
        inner: Box::new(WindowsWindowManager::new()),
      });
    }
    #[cfg(feature = "macos")]
    {
      use macos::MacosManger;
      let _ = try_create_manager!(MacosManger::new()).inspect_err(|e| tracing::error!("Failed creating X11 manager {e:?}"));
    }
    #[cfg(feature = "x11")]
    {
      use x11::X11WindowManager;
      let _ = try_create_manager!(X11WindowManager::new()).inspect_err(|e| tracing::error!("Failed creating X11 manager {e:?}"));
    }
    #[cfg(feature = "kde_wlnd")]
    {
      use kde_wayland::KdeWindowManager;
      let _ = try_create_manager!(KdeWindowManager::new().await).inspect_err(|e| tracing::error!("Failed creating KDE window manager {e:?}"));
    }
    #[cfg(feature = "gnome_wlnd")]
    {
      use gnome_wayland::GnomeWindowManager;
      let _ = try_create_manager!(GnomeWindowManager::new().await).inspect_err(|e| tracing::error!("Failed creating Gnome window manager {e:?}"));
    }
    #[cfg(feature = "wlr_wlnd")]
    {
      use wlr_management_wayland::WlrWindowManager;

      let _ = try_create_manager!(WlrWindowManager::new().await).inspect_err(|e| tracing::error!("Failed creating Wlr window manager {e:?}"));
    }

    // This runtime error is needed to allow the project to be compiled for during testing.
    return Err(anyhow!("No window was found matching"));
  }
}

#[async_trait]
impl ActiveWindowManager for GenericActiveWindowManager {
  async fn get_active_window_data(&mut self) -> Result<ActiveWindowData> {
    self.inner.get_active_window_data().await
  }
}
