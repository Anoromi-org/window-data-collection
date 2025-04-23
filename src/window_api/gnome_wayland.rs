use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use tracing::{debug, trace};
use zbus::Connection;

use super::{ActiveWindowData, ActiveWindowManager, WindowManager};

pub struct GnomeWindowManager {
    dbus_connection: Connection,
    last_title: String,
    last_app_id: String,
    last_pid: u32,
}

#[derive(Deserialize)]
struct GnomeWindowData {
    wm_class: String,
    title: String,
    pid: u32,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum EmptyOptional<T> {
    Optional(T),
    Empty {},
}

impl<T> From<EmptyOptional<T>> for Option<T> {
    fn from(value: EmptyOptional<T>) -> Self {
        match value {
            EmptyOptional::Optional(t) => Some(t),
            EmptyOptional::Empty {} => None,
        }
    }
}

impl GnomeWindowManager {
    async fn get_window_data(&self) -> anyhow::Result<Option<GnomeWindowData>> {
        let call_response = self
            .dbus_connection
            .call_method(
                Some("org.gnome.Shell"),
                "/org/gnome/shell/extensions/FocusedWindow",
                Some("org.gnome.shell.extensions.FocusedWindow"),
                "Get",
                &(),
            )
            .await;

        match call_response {
            Ok(json) => {
                let json: String = json
                    .body()
                    .deserialize()
                    .with_context(|| "DBus interface cannot be parsed as string")?;
                let data : EmptyOptional<GnomeWindowData> = serde_json::from_str(&json).with_context(|| {
                    format!("DBus interface org.gnome.shell.extensions.FocusedWindow returned wrong JSON: {json}")
                })?;
                Ok(data.into())
            }
            Err(e) => {
                if e.to_string().contains("No window in focus") {
                    trace!("No window is active");
                }
                Err(e.into())
            }
        }
    }
}

impl GnomeWindowManager {
    pub async fn new() -> anyhow::Result<Self> {
        let watcher = Self {
            dbus_connection: Connection::session().await?,
            last_app_id: String::new(),
            last_title: String::new(),
            last_pid: 0,
        };
        watcher.get_window_data().await?;

        Ok(watcher)
    }
}

#[async_trait]
impl ActiveWindowManager for GnomeWindowManager {
    async fn get_active_window_data(&mut self) -> Result<ActiveWindowData> {
        let data = self.get_window_data().await;
        if let Err(e) = data {
            if e.to_string().contains("Object does not exist at path") {
                trace!("The extension seems to have stopped");
            }
            return Err(e);
        }
        let Some(data) = data? else {
            return Err(anyhow!("No window is active"));
        };

        if data.wm_class != self.last_app_id
            || data.title != self.last_title
            || data.pid != self.last_pid
        {
            debug!(
                r#"Changed window app_id="{}", title="{}", pid="{}""#,
                data.wm_class, data.title, data.pid
            );
            self.last_app_id = data.wm_class.clone();
            self.last_title = data.title.clone();
            self.last_pid = data.pid;
        }

        let process_name = super::process_name::get(self.last_pid)?
            .ok_or_else(|| anyhow!("Failed to get process name: process name is None"))?;

        Ok(ActiveWindowData {
            window_title: data.title.into(),
            process_name: Some(process_name.into()),
            app_id: Some(data.wm_class.into()),
        })
    }
}
