// ==================================================
// Recursive expansion of the wayland_protocol! macro
// ==================================================

pub use self::generated::client;
mod generated {
  #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
  #![allow(non_upper_case_globals, non_snake_case, unused_imports)]
  #![allow(missing_docs, clippy::all)]
  pub mod client {
    #![doc = r" Client-side API of this protocol"]
    use wayland_client;
    use wayland_client::protocol::*;
    pub mod __interfaces {
      use wayland_client::protocol::__interfaces::*;
      use std::ptr::null;
      struct SyncWrapper<T>(T);
      unsafe impl<T> Sync for SyncWrapper<T> {}
      static types_null: SyncWrapper<[*const wayland_backend::protocol::wl_interface; 1]> =
        SyncWrapper([null::<wayland_backend::protocol::wl_interface>(); 1]);
      pub static EXT_FOREIGN_TOPLEVEL_LIST_V1_INTERFACE: wayland_backend::protocol::Interface =
        wayland_backend::protocol::Interface {
          name: "ext_foreign_toplevel_list_v1",
          version: 1u32,
          requests: &[
            wayland_backend::protocol::MessageDesc {
              name: "stop",
              signature: &[],
              since: 1u32,
              is_destructor: false,
              child_interface: None,
              arg_interfaces: &[],
            },
            wayland_backend::protocol::MessageDesc {
              name: "destroy",
              signature: &[],
              since: 1u32,
              is_destructor: true,
              child_interface: None,
              arg_interfaces: &[],
            },
          ],
          events: &[
            wayland_backend::protocol::MessageDesc {
              name: "toplevel",
              signature: &[wayland_backend::protocol::ArgumentType::NewId],
              since: 1u32,
              is_destructor: false,
              child_interface: Some(&EXT_FOREIGN_TOPLEVEL_HANDLE_V1_INTERFACE),
              arg_interfaces: &[],
            },
            wayland_backend::protocol::MessageDesc {
              name: "finished",
              signature: &[],
              since: 1u32,
              is_destructor: false,
              child_interface: None,
              arg_interfaces: &[],
            },
          ],
          c_ptr: Some(unsafe { &ext_foreign_toplevel_list_v1_interface }),
        };
      static ext_foreign_toplevel_list_v1_requests: SyncWrapper<[wayland_backend::protocol::wl_message; 2]> = SyncWrapper([
        wayland_backend::protocol::wl_message {
          name: b"stop\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
        wayland_backend::protocol::wl_message {
          name: b"destroy\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
      ]);
      static ext_foreign_toplevel_list_v1_events_toplevel_types: SyncWrapper<
        [*const wayland_backend::protocol::wl_interface; 1],
      > = SyncWrapper([&ext_foreign_toplevel_handle_v1_interface as *const wayland_backend::protocol::wl_interface]);
      static ext_foreign_toplevel_list_v1_events: SyncWrapper<[wayland_backend::protocol::wl_message; 2]> = SyncWrapper([
        wayland_backend::protocol::wl_message {
          name: b"toplevel\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"n\0" as *const u8 as *const std::os::raw::c_char,
          types: ext_foreign_toplevel_list_v1_events_toplevel_types.0.as_ptr(),
        },
        wayland_backend::protocol::wl_message {
          name: b"finished\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
      ]);
      pub static ext_foreign_toplevel_list_v1_interface: wayland_backend::protocol::wl_interface =
        wayland_backend::protocol::wl_interface {
          name: b"ext_foreign_toplevel_list_v1\0" as *const u8 as *const std::os::raw::c_char,
          version: 1,
          request_count: 2,
          requests: ext_foreign_toplevel_list_v1_requests.0.as_ptr(),
          event_count: 2,
          events: ext_foreign_toplevel_list_v1_events.0.as_ptr(),
        };
      pub static EXT_FOREIGN_TOPLEVEL_HANDLE_V1_INTERFACE: wayland_backend::protocol::Interface =
        wayland_backend::protocol::Interface {
          name: "ext_foreign_toplevel_handle_v1",
          version: 1u32,
          requests: &[wayland_backend::protocol::MessageDesc {
            name: "destroy",
            signature: &[],
            since: 1u32,
            is_destructor: true,
            child_interface: None,
            arg_interfaces: &[],
          }],
          events: &[
            wayland_backend::protocol::MessageDesc {
              name: "closed",
              signature: &[],
              since: 1u32,
              is_destructor: false,
              child_interface: None,
              arg_interfaces: &[],
            },
            wayland_backend::protocol::MessageDesc {
              name: "done",
              signature: &[],
              since: 1u32,
              is_destructor: false,
              child_interface: None,
              arg_interfaces: &[],
            },
            wayland_backend::protocol::MessageDesc {
              name: "title",
              signature: &[wayland_backend::protocol::ArgumentType::Str(
                wayland_backend::protocol::AllowNull::No,
              )],
              since: 1u32,
              is_destructor: false,
              child_interface: None,
              arg_interfaces: &[],
            },
            wayland_backend::protocol::MessageDesc {
              name: "app_id",
              signature: &[wayland_backend::protocol::ArgumentType::Str(
                wayland_backend::protocol::AllowNull::No,
              )],
              since: 1u32,
              is_destructor: false,
              child_interface: None,
              arg_interfaces: &[],
            },
            wayland_backend::protocol::MessageDesc {
              name: "identifier",
              signature: &[wayland_backend::protocol::ArgumentType::Str(
                wayland_backend::protocol::AllowNull::No,
              )],
              since: 1u32,
              is_destructor: false,
              child_interface: None,
              arg_interfaces: &[],
            },
          ],
          c_ptr: Some(unsafe { &ext_foreign_toplevel_handle_v1_interface }),
        };
      static ext_foreign_toplevel_handle_v1_requests: SyncWrapper<[wayland_backend::protocol::wl_message; 1]> =
        SyncWrapper([wayland_backend::protocol::wl_message {
          name: b"destroy\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        }]);
      static ext_foreign_toplevel_handle_v1_events: SyncWrapper<[wayland_backend::protocol::wl_message; 5]> = SyncWrapper([
        wayland_backend::protocol::wl_message {
          name: b"closed\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
        wayland_backend::protocol::wl_message {
          name: b"done\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
        wayland_backend::protocol::wl_message {
          name: b"title\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"s\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
        wayland_backend::protocol::wl_message {
          name: b"app_id\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"s\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
        wayland_backend::protocol::wl_message {
          name: b"identifier\0" as *const u8 as *const std::os::raw::c_char,
          signature: b"s\0" as *const u8 as *const std::os::raw::c_char,
          types: types_null.0.as_ptr(),
        },
      ]);
      pub static ext_foreign_toplevel_handle_v1_interface: wayland_backend::protocol::wl_interface =
        wayland_backend::protocol::wl_interface {
          name: b"ext_foreign_toplevel_handle_v1\0" as *const u8 as *const std::os::raw::c_char,
          version: 1,
          request_count: 1,
          requests: ext_foreign_toplevel_handle_v1_requests.0.as_ptr(),
          event_count: 5,
          events: ext_foreign_toplevel_handle_v1_events.0.as_ptr(),
        };
    }
    use self::__interfaces::*;
    #[doc = "list toplevels\n\nA toplevel is defined as a surface with a role similar to xdg_toplevel.\nXWayland surfaces may be treated like toplevels in this protocol.\n\nAfter a client binds the ext_foreign_toplevel_list_v1, each mapped\ntoplevel window will be sent using the ext_foreign_toplevel_list_v1.toplevel\nevent.\n\nClients which only care about the current state can perform a roundtrip after\nbinding this global.\n\nFor each instance of ext_foreign_toplevel_list_v1, the compositor must\ncreate a new ext_foreign_toplevel_handle_v1 object for each mapped toplevel.\n\nIf a compositor implementation sends the ext_foreign_toplevel_list_v1.finished\nevent after the global is bound, the compositor must not send any\next_foreign_toplevel_list_v1.toplevel events."]
    pub mod ext_foreign_toplevel_list_v1 {
      use std::sync::Arc;
      use std::os::unix::io::OwnedFd;
      use super::wayland_client::{
        backend::{
          Backend, WeakBackend, smallvec, ObjectData, ObjectId, InvalidId,
          protocol::{WEnum, Argument, Message, Interface, same_interface},
        },
        QueueProxyData, Proxy, Connection, Dispatch, QueueHandle, DispatchError, Weak,
      };
      #[doc = r" The minimal object version supporting this request"]
      pub const REQ_STOP_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this request"]
      pub const REQ_STOP_OPCODE: u16 = 0u16;
      #[doc = r" The minimal object version supporting this request"]
      pub const REQ_DESTROY_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this request"]
      pub const REQ_DESTROY_OPCODE: u16 = 1u16;
      #[doc = r" The minimal object version supporting this event"]
      pub const EVT_TOPLEVEL_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this event"]
      pub const EVT_TOPLEVEL_OPCODE: u16 = 0u16;
      #[doc = r" The minimal object version supporting this event"]
      pub const EVT_FINISHED_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this event"]
      pub const EVT_FINISHED_OPCODE: u16 = 1u16;
      #[derive(Debug)]
      #[non_exhaustive]
      pub enum Request<'a> {
        #[doc = "stop sending events\n\nThis request indicates that the client no longer wishes to receive\nevents for new toplevels.\n\nThe Wayland protocol is asynchronous, meaning the compositor may send\nfurther toplevel events until the stop request is processed.\nThe client should wait for a ext_foreign_toplevel_list_v1.finished\nevent before destroying this object."]
        Stop,
        #[doc = "destroy the ext_foreign_toplevel_list_v1 object\n\nThis request should be called either when the client will no longer\nuse the ext_foreign_toplevel_list_v1 or after the finished event\nhas been received to allow destruction of the object.\n\nIf a client wishes to destroy this object it should send a\next_foreign_toplevel_list_v1.stop request and wait for a ext_foreign_toplevel_list_v1.finished\nevent, then destroy the handles and then this object.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc(hidden)]
        __phantom_lifetime {
          phantom: std::marker::PhantomData<&'a ()>,
          never: std::convert::Infallible,
        },
      }
      impl<'a> Request<'a> {
        #[doc = "Get the opcode number of this message"]
        pub fn opcode(&self) -> u16 {
          match *self {
            Request::Stop => 0u16,
            Request::Destroy => 1u16,
            Request::__phantom_lifetime { never, .. } => match never {},
          }
        }
      }
      #[derive(Debug)]
      #[non_exhaustive]
      pub enum Event {
        #[doc = "a toplevel has been created\n\nThis event is emitted whenever a new toplevel window is created. It is\nemitted for all toplevels, regardless of the app that has created them.\n\nAll initial properties of the toplevel (identifier, title, app_id) will be sent\nimmediately after this event using the corresponding events for\next_foreign_toplevel_handle_v1. The compositor will use the\next_foreign_toplevel_handle_v1.done event to indicate when all data has\nbeen sent."]
        Toplevel {
          toplevel: super::ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        },
        #[doc = "the compositor has finished with the toplevel manager\n\nThis event indicates that the compositor is done sending events\nto this object. The client should destroy the object.\nSee ext_foreign_toplevel_list_v1.destroy for more information.\n\nThe compositor must not send any more toplevel events after this event."]
        Finished,
      }
      impl Event {
        #[doc = "Get the opcode number of this message"]
        pub fn opcode(&self) -> u16 {
          match *self {
            Event::Toplevel { .. } => 0u16,
            Event::Finished => 1u16,
          }
        }
      }
      #[doc = "list toplevels\n\nA toplevel is defined as a surface with a role similar to xdg_toplevel.\nXWayland surfaces may be treated like toplevels in this protocol.\n\nAfter a client binds the ext_foreign_toplevel_list_v1, each mapped\ntoplevel window will be sent using the ext_foreign_toplevel_list_v1.toplevel\nevent.\n\nClients which only care about the current state can perform a roundtrip after\nbinding this global.\n\nFor each instance of ext_foreign_toplevel_list_v1, the compositor must\ncreate a new ext_foreign_toplevel_handle_v1 object for each mapped toplevel.\n\nIf a compositor implementation sends the ext_foreign_toplevel_list_v1.finished\nevent after the global is bound, the compositor must not send any\next_foreign_toplevel_list_v1.toplevel events.\n\nSee also the [Event] enum for this interface."]
      #[derive(Debug, Clone)]
      pub struct ExtForeignToplevelListV1 {
        id: ObjectId,
        version: u32,
        data: Option<Arc<dyn ObjectData>>,
        backend: WeakBackend,
      }
      impl std::cmp::PartialEq for ExtForeignToplevelListV1 {
        fn eq(&self, other: &ExtForeignToplevelListV1) -> bool {
          self.id == other.id
        }
      }
      impl std::cmp::Eq for ExtForeignToplevelListV1 {}
      impl PartialEq<Weak<ExtForeignToplevelListV1>> for ExtForeignToplevelListV1 {
        fn eq(&self, other: &Weak<ExtForeignToplevelListV1>) -> bool {
          self.id == other.id()
        }
      }
      impl std::borrow::Borrow<ObjectId> for ExtForeignToplevelListV1 {
        fn borrow(&self) -> &ObjectId {
          &self.id
        }
      }
      impl std::hash::Hash for ExtForeignToplevelListV1 {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
          self.id.hash(state)
        }
      }
      impl super::wayland_client::Proxy for ExtForeignToplevelListV1 {
        type Request<'request> = Request<'request>;
        type Event = Event;
        #[inline]
        fn interface() -> &'static Interface {
          &super::EXT_FOREIGN_TOPLEVEL_LIST_V1_INTERFACE
        }
        #[inline]
        fn id(&self) -> ObjectId {
          self.id.clone()
        }
        #[inline]
        fn version(&self) -> u32 {
          self.version
        }
        #[inline]
        fn data<U: Send + Sync + 'static>(&self) -> Option<&U> {
          self.data.as_ref().and_then(|arc| arc.data_as_any().downcast_ref::<U>())
        }
        fn object_data(&self) -> Option<&Arc<dyn ObjectData>> {
          self.data.as_ref()
        }
        fn backend(&self) -> &WeakBackend {
          &self.backend
        }
        fn send_request(&self, req: Self::Request<'_>) -> Result<(), InvalidId> {
          let conn = Connection::from_backend(self.backend.upgrade().ok_or(InvalidId)?);
          let id = conn.send_request(self, req, None)?;
          if true {
            {
              if !(id.is_null()) {
                {
                  panic!()
                };
              }
            };
          };
          Ok(())
        }
        fn send_constructor<I: Proxy>(&self, req: Self::Request<'_>, data: Arc<dyn ObjectData>) -> Result<I, InvalidId> {
          let conn = Connection::from_backend(self.backend.upgrade().ok_or(InvalidId)?);
          let id = conn.send_request(self, req, Some(data))?;
          Proxy::from_id(&conn, id)
        }
        #[inline]
        fn from_id(conn: &Connection, id: ObjectId) -> Result<Self, InvalidId> {
          dbg!(id.interface(), Self::interface(), !id.is_null());
          if !same_interface(id.interface(), Self::interface()) && !id.is_null() {
            return Err(InvalidId);
          }
          let version = conn.object_info(id.clone()).map(|info| info.version).unwrap_or(0);
          let data = conn.get_object_data(id.clone()).ok();
          let backend = conn.backend().downgrade();
          Ok(ExtForeignToplevelListV1 {
            id,
            data,
            version,
            backend,
          })
        }
        #[inline]
        fn inert(backend: WeakBackend) -> Self {
          ExtForeignToplevelListV1 {
            id: ObjectId::null(),
            data: None,
            version: 0,
            backend,
          }
        }
        fn parse_event(conn: &Connection, msg: Message<ObjectId, OwnedFd>) -> Result<(Self, Self::Event), DispatchError> {
          let me = Self::from_id(conn, msg.sender_id.clone()).unwrap();
          let mut arg_iter = msg.args.into_iter();
          match msg.opcode {
            0u16 => {
              if let (Some(Argument::NewId(toplevel))) = (arg_iter.next()) {
                Ok((
                  me,
                  Event::Toplevel {
                    toplevel: match <super::ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1 as Proxy>::from_id(
                      conn,
                      toplevel.clone(),
                    ) {
                      Ok(p) => p,
                      Err(_) => {
                        return Err(DispatchError::BadMessage {
                          sender_id: msg.sender_id,
                          interface: Self::interface().name,
                          opcode: msg.opcode,
                        })
                      }
                    },
                  },
                ))
              } else {
                Err(DispatchError::BadMessage {
                  sender_id: msg.sender_id,
                  interface: Self::interface().name,
                  opcode: msg.opcode,
                })
              }
            }
            1u16 => {
              if let () = () {
                Ok((me, Event::Finished {}))
              } else {
                Err(DispatchError::BadMessage {
                  sender_id: msg.sender_id,
                  interface: Self::interface().name,
                  opcode: msg.opcode,
                })
              }
            }
            _ => Err(DispatchError::BadMessage {
              sender_id: msg.sender_id,
              interface: Self::interface().name,
              opcode: msg.opcode,
            }),
          }
        }
        fn write_request<'a>(
          &self,
          conn: &Connection,
          msg: Self::Request<'a>,
        ) -> Result<
          (
            Message<ObjectId, std::os::unix::io::BorrowedFd<'a>>,
            Option<(&'static Interface, u32)>,
          ),
          InvalidId,
        > {
          match msg {
            Request::Stop {} => {
              let child_spec = None;
              let args = smallvec::SmallVec::new();
              Ok((
                Message {
                  sender_id: self.id.clone(),
                  opcode: 0u16,
                  args,
                },
                child_spec,
              ))
            }
            Request::Destroy {} => {
              let child_spec = None;
              let args = smallvec::SmallVec::new();
              Ok((
                Message {
                  sender_id: self.id.clone(),
                  opcode: 1u16,
                  args,
                },
                child_spec,
              ))
            }
            Request::__phantom_lifetime { never, .. } => match never {},
          }
        }
      }
      impl ExtForeignToplevelListV1 {
        #[doc = "stop sending events\n\nThis request indicates that the client no longer wishes to receive\nevents for new toplevels.\n\nThe Wayland protocol is asynchronous, meaning the compositor may send\nfurther toplevel events until the stop request is processed.\nThe client should wait for a ext_foreign_toplevel_list_v1.finished\nevent before destroying this object."]
        #[allow(clippy::too_many_arguments)]
        pub fn stop(&self) {
          let backend = match self.backend.upgrade() {
            Some(b) => b,
            None => return,
          };
          let conn = Connection::from_backend(backend);
          let _ = conn.send_request(self, Request::Stop {}, None);
        }
        #[doc = "destroy the ext_foreign_toplevel_list_v1 object\n\nThis request should be called either when the client will no longer\nuse the ext_foreign_toplevel_list_v1 or after the finished event\nhas been received to allow destruction of the object.\n\nIf a client wishes to destroy this object it should send a\next_foreign_toplevel_list_v1.stop request and wait for a ext_foreign_toplevel_list_v1.finished\nevent, then destroy the handles and then this object."]
        #[allow(clippy::too_many_arguments)]
        pub fn destroy(&self) {
          let backend = match self.backend.upgrade() {
            Some(b) => b,
            None => return,
          };
          let conn = Connection::from_backend(backend);
          let _ = conn.send_request(self, Request::Destroy {}, None);
        }
      }
    }
    #[doc = "a mapped toplevel\n\nA ext_foreign_toplevel_handle_v1 object represents a mapped toplevel\nwindow. A single app may have multiple mapped toplevels."]
    pub mod ext_foreign_toplevel_handle_v1 {
      use std::sync::Arc;
      use std::os::unix::io::OwnedFd;
      use super::wayland_client::{
        backend::{
          Backend, WeakBackend, smallvec, ObjectData, ObjectId, InvalidId,
          protocol::{WEnum, Argument, Message, Interface, same_interface},
        },
        QueueProxyData, Proxy, Connection, Dispatch, QueueHandle, DispatchError, Weak,
      };
      #[doc = r" The minimal object version supporting this request"]
      pub const REQ_DESTROY_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this request"]
      pub const REQ_DESTROY_OPCODE: u16 = 0u16;
      #[doc = r" The minimal object version supporting this event"]
      pub const EVT_CLOSED_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this event"]
      pub const EVT_CLOSED_OPCODE: u16 = 0u16;
      #[doc = r" The minimal object version supporting this event"]
      pub const EVT_DONE_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this event"]
      pub const EVT_DONE_OPCODE: u16 = 1u16;
      #[doc = r" The minimal object version supporting this event"]
      pub const EVT_TITLE_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this event"]
      pub const EVT_TITLE_OPCODE: u16 = 2u16;
      #[doc = r" The minimal object version supporting this event"]
      pub const EVT_APP_ID_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this event"]
      pub const EVT_APP_ID_OPCODE: u16 = 3u16;
      #[doc = r" The minimal object version supporting this event"]
      pub const EVT_IDENTIFIER_SINCE: u32 = 1u32;
      #[doc = r" The wire opcode for this event"]
      pub const EVT_IDENTIFIER_OPCODE: u16 = 4u16;
      #[derive(Debug)]
      #[non_exhaustive]
      pub enum Request<'a> {
        #[doc = "destroy the ext_foreign_toplevel_handle_v1 object\n\nThis request should be used when the client will no longer use the handle\nor after the closed event has been received to allow destruction of the\nobject.\n\nWhen a handle is destroyed, a new handle may not be created by the server\nuntil the toplevel is unmapped and then remapped. Destroying a toplevel handle\nis not recommended unless the client is cleaning up child objects\nbefore destroying the ext_foreign_toplevel_list_v1 object, the toplevel\nwas closed or the toplevel handle will not be used in the future.\n\nOther protocols which extend the ext_foreign_toplevel_handle_v1\ninterface should require destructors for extension interfaces be\ncalled before allowing the toplevel handle to be destroyed.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc(hidden)]
        __phantom_lifetime {
          phantom: std::marker::PhantomData<&'a ()>,
          never: std::convert::Infallible,
        },
      }
      impl<'a> Request<'a> {
        #[doc = "Get the opcode number of this message"]
        pub fn opcode(&self) -> u16 {
          match *self {
            Request::Destroy => 0u16,
            Request::__phantom_lifetime { never, .. } => match never {},
          }
        }
      }
      #[derive(Debug)]
      #[non_exhaustive]
      pub enum Event {
        #[doc = "the toplevel has been closed\n\nThe server will emit no further events on the ext_foreign_toplevel_handle_v1\nafter this event. Any requests received aside from the destroy request must\nbe ignored. Upon receiving this event, the client should destroy the handle.\n\nOther protocols which extend the ext_foreign_toplevel_handle_v1\ninterface must also ignore requests other than destructors."]
        Closed,
        #[doc = "all information about the toplevel has been sent\n\nThis event is sent after all changes in the toplevel state have\nbeen sent.\n\nThis allows changes to the ext_foreign_toplevel_handle_v1 properties\nto be atomically applied. Other protocols which extend the\next_foreign_toplevel_handle_v1 interface may use this event to also\natomically apply any pending state.\n\nThis event must not be sent after the ext_foreign_toplevel_handle_v1.closed\nevent."]
        Done,
        #[doc = "title change\n\nThe title of the toplevel has changed.\n\nThe configured state must not be applied immediately. See\next_foreign_toplevel_handle_v1.done for details."]
        Title { title: String },
        #[doc = "app_id change\n\nThe app id of the toplevel has changed.\n\nThe configured state must not be applied immediately. See\next_foreign_toplevel_handle_v1.done for details."]
        AppId { app_id: String },
        #[doc = "a stable identifier for a toplevel\n\nThis identifier is used to check if two or more toplevel handles belong\nto the same toplevel.\n\nThe identifier is useful for command line tools or privileged clients\nwhich may need to reference an exact toplevel across processes or\ninstances of the ext_foreign_toplevel_list_v1 global.\n\nThe compositor must only send this event when the handle is created.\n\nThe identifier must be unique per toplevel and it's handles. Two different\ntoplevels must not have the same identifier. The identifier is only valid\nas long as the toplevel is mapped. If the toplevel is unmapped the identifier\nmust not be reused. An identifier must not be reused by the compositor to\nensure there are no races when sharing identifiers between processes.\n\nAn identifier is a string that contains up to 32 printable ASCII bytes.\nAn identifier must not be an empty string. It is recommended that a\ncompositor includes an opaque generation value in identifiers. How the\ngeneration value is used when generating the identifier is implementation\ndependent."]
        Identifier { identifier: String },
      }
      impl Event {
        #[doc = "Get the opcode number of this message"]
        pub fn opcode(&self) -> u16 {
          match *self {
            Event::Closed => 0u16,
            Event::Done => 1u16,
            Event::Title { .. } => 2u16,
            Event::AppId { .. } => 3u16,
            Event::Identifier { .. } => 4u16,
          }
        }
      }
      #[doc = "a mapped toplevel\n\nA ext_foreign_toplevel_handle_v1 object represents a mapped toplevel\nwindow. A single app may have multiple mapped toplevels.\n\nSee also the [Event] enum for this interface."]
      #[derive(Debug, Clone)]
      pub struct ExtForeignToplevelHandleV1 {
        id: ObjectId,
        version: u32,
        data: Option<Arc<dyn ObjectData>>,
        backend: WeakBackend,
      }
      impl std::cmp::PartialEq for ExtForeignToplevelHandleV1 {
        fn eq(&self, other: &ExtForeignToplevelHandleV1) -> bool {
          self.id == other.id
        }
      }
      impl std::cmp::Eq for ExtForeignToplevelHandleV1 {}
      impl PartialEq<Weak<ExtForeignToplevelHandleV1>> for ExtForeignToplevelHandleV1 {
        fn eq(&self, other: &Weak<ExtForeignToplevelHandleV1>) -> bool {
          self.id == other.id()
        }
      }
      impl std::borrow::Borrow<ObjectId> for ExtForeignToplevelHandleV1 {
        fn borrow(&self) -> &ObjectId {
          &self.id
        }
      }
      impl std::hash::Hash for ExtForeignToplevelHandleV1 {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
          self.id.hash(state)
        }
      }
      impl super::wayland_client::Proxy for ExtForeignToplevelHandleV1 {
        type Request<'request> = Request<'request>;
        type Event = Event;
        #[inline]
        fn interface() -> &'static Interface {
          &super::EXT_FOREIGN_TOPLEVEL_HANDLE_V1_INTERFACE
        }
        #[inline]
        fn id(&self) -> ObjectId {
          self.id.clone()
        }
        #[inline]
        fn version(&self) -> u32 {
          self.version
        }
        #[inline]
        fn data<U: Send + Sync + 'static>(&self) -> Option<&U> {
          self.data.as_ref().and_then(|arc| arc.data_as_any().downcast_ref::<U>())
        }
        fn object_data(&self) -> Option<&Arc<dyn ObjectData>> {
          self.data.as_ref()
        }
        fn backend(&self) -> &WeakBackend {
          &self.backend
        }
        fn send_request(&self, req: Self::Request<'_>) -> Result<(), InvalidId> {
          let conn = Connection::from_backend(self.backend.upgrade().ok_or(InvalidId)?);
          let id = conn.send_request(self, req, None)?;
          if true {
            {
              if !(id.is_null()) {
                {
                  panic!()
                };
              }
            };
          };
          Ok(())
        }
        fn send_constructor<I: Proxy>(&self, req: Self::Request<'_>, data: Arc<dyn ObjectData>) -> Result<I, InvalidId> {
          let conn = Connection::from_backend(self.backend.upgrade().ok_or(InvalidId)?);
          let id = conn.send_request(self, req, Some(data))?;
          Proxy::from_id(&conn, id)
        }
        #[inline]
        fn from_id(conn: &Connection, id: ObjectId) -> Result<Self, InvalidId> {
          if !same_interface(id.interface(), Self::interface()) && !id.is_null() {
            return Err(InvalidId);
          }
          let version = conn.object_info(id.clone()).map(|info| info.version).unwrap_or(0);
          let data = conn.get_object_data(id.clone()).ok();
          let backend = conn.backend().downgrade();
          Ok(ExtForeignToplevelHandleV1 {
            id,
            data,
            version,
            backend,
          })
        }
        #[inline]
        fn inert(backend: WeakBackend) -> Self {
          ExtForeignToplevelHandleV1 {
            id: ObjectId::null(),
            data: None,
            version: 0,
            backend,
          }
        }
        fn parse_event(conn: &Connection, msg: Message<ObjectId, OwnedFd>) -> Result<(Self, Self::Event), DispatchError> {
          let me = Self::from_id(conn, msg.sender_id.clone()).unwrap();
          let mut arg_iter = msg.args.into_iter();
          match msg.opcode {
            0u16 => {
              if let () = () {
                Ok((me, Event::Closed {}))
              } else {
                Err(DispatchError::BadMessage {
                  sender_id: msg.sender_id,
                  interface: Self::interface().name,
                  opcode: msg.opcode,
                })
              }
            }
            1u16 => {
              if let () = () {
                Ok((me, Event::Done {}))
              } else {
                Err(DispatchError::BadMessage {
                  sender_id: msg.sender_id,
                  interface: Self::interface().name,
                  opcode: msg.opcode,
                })
              }
            }
            2u16 => {
              if let (Some(Argument::Str(title))) = (arg_iter.next()) {
                Ok((
                  me,
                  Event::Title {
                    title: String::from_utf8_lossy(title.as_ref().unwrap().as_bytes()).into_owned(),
                  },
                ))
              } else {
                Err(DispatchError::BadMessage {
                  sender_id: msg.sender_id,
                  interface: Self::interface().name,
                  opcode: msg.opcode,
                })
              }
            }
            3u16 => {
              if let (Some(Argument::Str(app_id))) = (arg_iter.next()) {
                Ok((
                  me,
                  Event::AppId {
                    app_id: String::from_utf8_lossy(app_id.as_ref().unwrap().as_bytes()).into_owned(),
                  },
                ))
              } else {
                Err(DispatchError::BadMessage {
                  sender_id: msg.sender_id,
                  interface: Self::interface().name,
                  opcode: msg.opcode,
                })
              }
            }
            4u16 => {
              if let (Some(Argument::Str(identifier))) = (arg_iter.next()) {
                Ok((
                  me,
                  Event::Identifier {
                    identifier: String::from_utf8_lossy(identifier.as_ref().unwrap().as_bytes()).into_owned(),
                  },
                ))
              } else {
                Err(DispatchError::BadMessage {
                  sender_id: msg.sender_id,
                  interface: Self::interface().name,
                  opcode: msg.opcode,
                })
              }
            }
            _ => Err(DispatchError::BadMessage {
              sender_id: msg.sender_id,
              interface: Self::interface().name,
              opcode: msg.opcode,
            }),
          }
        }
        fn write_request<'a>(
          &self,
          conn: &Connection,
          msg: Self::Request<'a>,
        ) -> Result<
          (
            Message<ObjectId, std::os::unix::io::BorrowedFd<'a>>,
            Option<(&'static Interface, u32)>,
          ),
          InvalidId,
        > {
          match msg {
            Request::Destroy {} => {
              let child_spec = None;
              let args = smallvec::SmallVec::new();
              Ok((
                Message {
                  sender_id: self.id.clone(),
                  opcode: 0u16,
                  args,
                },
                child_spec,
              ))
            }
            Request::__phantom_lifetime { never, .. } => match never {},
          }
        }
      }
      impl ExtForeignToplevelHandleV1 {
        #[doc = "destroy the ext_foreign_toplevel_handle_v1 object\n\nThis request should be used when the client will no longer use the handle\nor after the closed event has been received to allow destruction of the\nobject.\n\nWhen a handle is destroyed, a new handle may not be created by the server\nuntil the toplevel is unmapped and then remapped. Destroying a toplevel handle\nis not recommended unless the client is cleaning up child objects\nbefore destroying the ext_foreign_toplevel_list_v1 object, the toplevel\nwas closed or the toplevel handle will not be used in the future.\n\nOther protocols which extend the ext_foreign_toplevel_handle_v1\ninterface should require destructors for extension interfaces be\ncalled before allowing the toplevel handle to be destroyed."]
        #[allow(clippy::too_many_arguments)]
        pub fn destroy(&self) {
          let backend = match self.backend.upgrade() {
            Some(b) => b,
            None => return,
          };
          let conn = Connection::from_backend(backend);
          let _ = conn.send_request(self, Request::Destroy {}, None);
        }
      }
    }
  }
}
