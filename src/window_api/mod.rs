//! Contains logic for extracting records from different environments.
//! [GenericWindowManager] is the main artifact of this module that abstracts
//! the operations.

#[cfg(feature = "ext_wlnd")]
pub mod ext_list_wayland;
#[cfg(feature = "gnome_wlnd")]
pub mod gnome_wayland;
#[cfg(feature = "kde_wlnd")]
pub mod kde_wayland;
pub mod process_name;
#[cfg(feature = "win")]
pub mod win;
#[cfg(any(feature = "wlr_wlnd", feature = "ext_wlnd"))]
pub mod wl_connection;
#[cfg(feature = "wlr_wlnd")]
pub mod wlr_management_wayland;
#[cfg(feature = "x11")]
pub mod x11;
pub mod test;

#[cfg(feature = "win")]
extern crate windows;

#[cfg(feature = "x11")]
extern crate xcb;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::error;

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
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait WindowManager {
    async fn get_active_window_data(&mut self) -> Result<ActiveWindowData>;

    /// Retrieve amount of time user has been inactive in milliseconds
    async fn get_idle_time(&mut self) -> Result<u32>;
}

/// Serves as a cross-compatible WindowManager implementation.
pub struct GenericWindowManager {
    inner: Box<dyn WindowManager + Send>,
}

impl GenericWindowManager {
    pub async fn new() -> Result<Self> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "win")] {
                use win::WindowsWindowManager;
                Ok(Self {
                    inner: Box::new(WindowsWindowManager::new()),
                })
            }
            else if #[cfg(feature = "x11")] {
                use x11::LinuxWindowManager;
                Ok(Self {
                    inner: Box::new(LinuxWindowManager::new()?),
                })
            }
            else if #[cfg(feature = "kde_wlnd")] {
                use kde_wayland::WindowWatcher;
                Ok(Self {
                    inner: Box::new(WindowWatcher::new().await?),
                })
            }
            else if #[cfg(feature = "gnome_wlnd")] {
                use gnome_wayland::GnomeWindowManager;

                Ok(Self {
                    inner: Box::new(GnomeWindowManager::new().await?),
                })
            }
            else if #[cfg(feature = "wlr_wlnd")] {
                use wlr_management_wayland::WlrWindowManager;

                Ok(Self {
                    inner: Box::new(WlrWindowManager::new().await.inspect_err(|e| error!("Failed to create wlr window manager: {e:?}"))?),
                })
            }
            else if #[cfg(feature = "ext_wlnd")] {
                use ext_list_wayland::ExtWindowManager;

                Ok(Self {
                    inner: Box::new(ExtWindowManager::new().await?),
                })
            }
            else {
                // This runtime error is needed to allow the project to be compiled for during testing.
                unimplemented!("No window manager was specified")
            }
        }
    }
}

#[async_trait]
impl WindowManager for GenericWindowManager {
    async fn get_active_window_data(&mut self) -> Result<ActiveWindowData> {
        self.inner.get_active_window_data().await
    }

    async fn get_idle_time(&mut self) -> Result<u32> {
        self.inner.get_idle_time().await
    }
}
