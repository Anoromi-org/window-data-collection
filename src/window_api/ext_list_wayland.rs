use super::wl_connection::WlEventConnection;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{debug, error, trace, warn};
use wayland_client::{
    event_created_child, globals::GlobalListContents, protocol::wl_registry, Connection, Dispatch,
    Proxy, QueueHandle,
};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_handle_v1::{
    Event as HandleEvent, ExtForeignToplevelHandleV1,
};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_list_v1::{
    Event as ListEvent, ExtForeignToplevelListV1, EVT_TOPLEVEL_OPCODE,
};

use super::{ActiveWindowData, WindowManager};

struct WindowData {
    app_id: String,
    title: String,
}

struct ToplevelState {
    windows: HashMap<String, WindowData>,
    current_window_id: Option<String>,
}

impl ToplevelState {
    fn new() -> Self {
        Self {
            windows: HashMap::new(),
            current_window_id: None,
        }
    }
}

impl Dispatch<ExtForeignToplevelListV1, ()> for ToplevelState {
    fn event(
        state: &mut Self,
        _: &ExtForeignToplevelListV1,
        event: <ExtForeignToplevelListV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            ListEvent::Toplevel { toplevel } => {
                debug!("Toplevel handle is received {}", toplevel.id());
                state.windows.insert(
                    toplevel.id().to_string(),
                    WindowData {
                        app_id: "unknown".into(),
                        title: "unknown".into(),
                    },
                );
            }
            ListEvent::Finished => {
                error!("Toplevel manager is finished, the application may crash");
            }
            _ => (),
        };
    }

    event_created_child!(ToplevelState, ExtForeignToplevelListV1, [
        EVT_TOPLEVEL_OPCODE => (ExtForeignToplevelListV1, ()),
    ]);
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for ToplevelState {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: <wl_registry::WlRegistry as Proxy>::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for ToplevelState {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: <wl_registry::WlRegistry as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ExtForeignToplevelHandleV1, ()> for ToplevelState {
    fn event(
        toplevel_state: &mut Self,
        handle: &ExtForeignToplevelHandleV1,
        event: <ExtForeignToplevelHandleV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let id = handle.id().to_string();
        let window = toplevel_state.windows.get_mut(&id);
        if let Some(window) = window {
            match event {
                HandleEvent::Title { title } => {
                    trace!("Title is changed for {id}: {title}");
                    window.title = title;
                }
                HandleEvent::AppId { app_id } => {
                    trace!("App ID is changed for {id}: {app_id}");
                    window.app_id = app_id;
                }
                HandleEvent::Identifier { identifier } => {
                    trace!("State is changed for {id}: {:?}", identifier);
                    toplevel_state.current_window_id = Some(id);
                    // if state.contains(&(HandleState::Activated as u8)) {
                    //     trace!("Window is activated: {id}");
                    //     toplevel_state.current_window_id = Some(id);
                    // }
                }
                HandleEvent::Done => trace!("Done: {id}"),
                HandleEvent::Closed => {
                    trace!("Window is closed: {id}");
                    if toplevel_state.windows.remove(&id).is_none() {
                        warn!("Window is already removed: {id}");
                    }
                }
                _ => (),
            };
        } else {
            error!("Window is not found: {id}");
        }
    }
}

pub struct ExtWindowManager {
    connection: WlEventConnection<ToplevelState>,
    toplevel_state: ToplevelState,
}

impl ExtWindowManager {
    pub async fn new() -> anyhow::Result<Self> {
        let mut connection: WlEventConnection<ToplevelState> = WlEventConnection::connect()?;
        connection.get_foreign_toplevel_list()?; 
                    
        let mut toplevel_state = ToplevelState::new();

        connection
            .event_queue
            .roundtrip(&mut toplevel_state)
            .unwrap();

        Ok(Self {
            connection,
            toplevel_state,
        })
    }
}

#[async_trait]
impl WindowManager for ExtWindowManager {
    async fn get_active_window_data(&mut self) -> Result<ActiveWindowData> {
        self.connection
            .event_queue
            .roundtrip(&mut self.toplevel_state)
            .map_err(|e| anyhow!("Event queue is not processed: {e}"))?;

        let active_window_id = self
            .toplevel_state
            .current_window_id
            .as_ref()
            .ok_or(anyhow!("Current window is unknown"))?;
        let active_window = self
            .toplevel_state
            .windows
            .get(active_window_id)
            .ok_or(anyhow!(
                "Current window is not found by ID {active_window_id}"
            ))?;

        Ok(ActiveWindowData {
            window_title: active_window.title.clone().into(),
            process_name: None,
            app_id: Some(active_window.app_id.clone().into()),
        })
    }

    /// Retrieve amount of time user has been inactive in milliseconds
    async fn get_idle_time(&mut self) -> Result<u32> {
        Ok(0)
    }
}
