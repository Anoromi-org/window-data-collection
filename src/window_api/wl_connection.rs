use anyhow::Context;
use wayland_client::
{
  globals::{ registry_queue_init, GlobalList, GlobalListContents },
  protocol::wl_registry,
  Connection,
  Dispatch,
  EventQueue,
  Proxy,
  QueueHandle,
};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1;
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_manager_v1::ZwlrForeignToplevelManagerV1;

pub struct WlEventConnection< T >
{
  pub globals : GlobalList,
  pub event_queue : EventQueue< T >,
  pub queue_handle : QueueHandle< T >,
}

impl< T > WlEventConnection< T >
where
  T : Dispatch< wl_registry::WlRegistry, GlobalListContents > + Dispatch< wl_registry::WlRegistry, () > + 'static,
{
  pub fn connect() -> anyhow::Result< Self >
  {
    let connection = Connection::connect_to_env().with_context( || "Unable to connect to Wayland compositor" )?;
    let display = connection.display();
    let ( globals, event_queue ) = registry_queue_init::< T >( &connection )?;

    let queue_handle = event_queue.handle();

    let _registry = display.get_registry( &queue_handle, () );

    Ok( Self{ globals, event_queue, queue_handle } )
  }

  pub fn get_foreign_toplevel_manager( &self ) -> anyhow::Result< ZwlrForeignToplevelManagerV1 >
  where
    T : Dispatch< ZwlrForeignToplevelManagerV1, () >,
  {
    self
      .globals
      .bind::< ZwlrForeignToplevelManagerV1, T, () >
      (
        &self.queue_handle,
        1 ..= ZwlrForeignToplevelManagerV1::interface().version,
        (),
      )
      .map_err( std::convert::Into::into )
  }

  pub fn get_foreign_toplevel_list( &self ) -> anyhow::Result< ExtForeignToplevelListV1 >
  where
    T : Dispatch< ExtForeignToplevelListV1, () >,
  {
    self
      .globals
      .bind::< ExtForeignToplevelListV1, T, () >( &self.queue_handle, 1 ..= ExtForeignToplevelListV1::interface().version, () )
      .map_err( std::convert::Into::into )
  }

}
