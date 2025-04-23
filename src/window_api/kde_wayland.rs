/*
 * This uses a hack with KWin scripts in order to receive the active window.
 * For the moment of writing, KWin doesn't implement the appropriate protocols to get a top level window.
 * Inspired by https://github.com/k0kubun/xremap/
 */
use anyhow::{ anyhow, Context, Result };
use std::env::{ self, temp_dir };
use std::path::Path;
use std::sync::Mutex;
use std::sync::{ mpsc::channel, Arc };
use std::thread;
use tracing::{ debug, error };
use zbus::interface;
use zbus::blocking::{ connection::Builder as ConnectionBuilder, Connection };

use super::{ ActiveWindowData, ActiveWindowManager };

const KWIN_SCRIPT_NAME : &str = "window_data_collection";
const KWIN_SCRIPT : &str = include_str!( "./kde_wayland.js" );

struct KWinScript
{
  dbus_connection : Connection,
  is_loaded : bool,
}

impl KWinScript
{
  fn new( dbus_connection : Connection ) -> Self
  {
    KWinScript
    {
      dbus_connection,
      is_loaded : false,
    }
  }

   fn load( &mut self ) -> anyhow::Result< () >
  {
    let path = temp_dir().join( "kwin_window.js" );
    std::fs::write( &path, KWIN_SCRIPT ).unwrap();

    let number = self.get_registered_number( &path )?;
    let result = self.start( number );
    std::fs::remove_file( &path )?;
    self.is_loaded = true;

    result
  }

   fn is_loaded( &self ) -> anyhow::Result< bool >
  {
    self
    .dbus_connection
    .call_method
    (
      Some( "org.kde.KWin" ),
      "/Scripting",
      Some( "org.kde.kwin.Scripting" ),
      "isScriptLoaded",
      &KWIN_SCRIPT_NAME,
    )
    ?
    .body()
    .deserialize()
    .map_err( std::convert::Into::into )
  }

   fn unload_kwin( connection : &Connection ) -> Result< bool >
  {
    connection
    .call_method
    (
      Some( "org.kde.KWin" ),
      "/Scripting",
      Some( "org.kde.kwin.Scripting" ),
      "unloadScript",
      &KWIN_SCRIPT_NAME,
    )
    ?
    .body()
    .deserialize()
    .map_err( std::convert::Into::into )
  }

   fn get_registered_number( &self, path : &Path ) -> anyhow::Result< i32 >
  {
    let temp_path = path.to_str().ok_or( anyhow!( "Temporary file path is not valid" ) )?;

    self
    .dbus_connection
    .call_method
    (
      Some( "org.kde.KWin" ),
      "/Scripting",
      Some( "org.kde.kwin.Scripting" ),
      "loadScript",
      // since OsStr does not implement zvariant::Type, the temp-path must be valid utf-8
      &( temp_path, KWIN_SCRIPT_NAME ),
    )
    ?
    .body()
    .deserialize()
    .map_err( std::convert::Into::into )
  }

   fn unload( &self ) -> anyhow::Result< bool > { Self::unload_kwin( &self.dbus_connection ) }

   fn start( &self, script_number : i32 ) -> anyhow::Result< () >
  {
    debug!( "Starting KWin script {script_number}" );

    let path = if self.get_major_version() < 6
    {
      format!( "/{script_number}" )
    }
    else
    {
      format!( "/Scripting/Script{script_number}" )
    };
    self
      .dbus_connection
      .call_method( Some( "org.kde.KWin" ), path, Some( "org.kde.kwin.Script" ), "run", &() )
      
      .with_context( || "Error on starting the script" )?;
    Ok( () )
  }

   fn get_major_version( &self ) -> i8
  {
    if let Ok( version ) = Self::get_major_version_from_env()
    {
      debug!( "KWin version from KDE_SESSION_VERSION: {version}" );

      version
    }
    else
    {
      self
      .get_major_version_from_dbus()
      
      .unwrap_or_else
      ( 
        | e | 
        {
          error!( "Failed to get KWin version: {e}" );
          5
        } 
      )
    }
  }

  fn get_major_version_from_env() -> anyhow::Result< i8 >
  {
    env::var( "KDE_SESSION_VERSION" )?
    .parse::<i8>()
    .map_err( std::convert::Into::into )
  }

   fn get_major_version_from_dbus( &self ) -> anyhow::Result< i8 >
  {
    let support_information : String = self
    .dbus_connection
    .call_method
    (
      Some( "org.kde.KWin" ),
      "/KWin",
      Some( "org.kde.KWin" ),
      "supportInformation",
      &(),
    )
    ?
    .body()
    .deserialize()?;

    // find a string like "KWin version: 5.27.8" and extract the version number from it:
    let version = support_information
    .lines()
    .find( | line | line.starts_with( "KWin version: " ) )
    .ok_or( anyhow!( "KWin version not found" ) )?
    .split_whitespace()
    .last()
    .ok_or( anyhow!( "KWin version is invalid" ) )?;

    // Extract the major version number from the version number like "5.27.8":
    let major_version = version
    .split( '.' )
    .next()
    .ok_or( anyhow!( "KWin version is invalid: {version}" ) )?
    .parse::<i8>()?;

    debug!( "KWin version from DBus: {version}, major version: {major_version}" );

    Ok( major_version )
  }
}

impl Drop for KWinScript
{
  fn drop( &mut self )
  {
    if self.is_loaded
    {
      let connection = self.dbus_connection.clone();
      let _ = Self::unload_kwin( &connection )
      .inspect_err( | e | error!( "Failed to unload KWin script: {e}" ) );
    }
  }
}

struct ActiveWindow
{
  resource_class : String,
  resource_name : String,
  caption : String,
}

struct ActiveWindowInterface
{
  active_window : Arc< Mutex< ActiveWindow > >,
}

#[ interface( name = "systems.obox.window_data_collection" ) ]
impl ActiveWindowInterface
{
   fn notify_active_window( &mut self, caption : String, resource_class : String, resource_name : String )
  {
    debug!( "Active window class: \"{resource_class}\", name: \"{resource_name}\", caption: \"{caption}\"" );
    let mut active_window = self.active_window.lock().unwrap();
    active_window.caption = caption;
    active_window.resource_class = resource_class;
    active_window.resource_name = resource_name;
  }
}

pub struct KdeWindowManager
{
  active_window : Arc< Mutex< ActiveWindow > >,
  // Prolong its lifetime
  _kwin_script : KWinScript,
}

impl KdeWindowManager
{
  pub  fn new() -> anyhow::Result< Self >
  {
    let mut kwin_script = KWinScript::new( Connection::session()? );
    if kwin_script.is_loaded()?
    {
      debug!( "KWin script is already loaded, unloading" );
      kwin_script.unload()?;
    }
    if env::var( "WAYLAND_DISPLAY" ).is_err() && env::var_os( "XDG_SESSION_TYPE" ).unwrap_or( "".into() ) == "x11"
    {
      return Err( anyhow!( "X11 should be tried instead" ) );
    }

    kwin_script.load().unwrap();

    let active_window = ActiveWindow
    {
      caption : String::new(),
      resource_name : String::new(),
      resource_class : String::new(),
    };
    let active_window = Arc::new( Mutex::new( active_window ) );
    let active_window_interface = ActiveWindowInterface{
      active_window : Arc::clone( &active_window ),
    };

    let ( tx, rx ) = channel();
    thread::spawn( move || {
       fn get_connection( active_window_interface : ActiveWindowInterface ) -> zbus::Result< Connection >
      {
        ConnectionBuilder::session()?
          .name( "systems.obox.window_data_collection" )?
          .serve_at( "/systems/obox/window_data_collection", active_window_interface )?
          .build()
          
      }

          match get_connection( active_window_interface )
          {
            Ok( connection ) =>
            {
              tx.send( None ).unwrap();
              loop
              {
                connection.monitor_activity();
              }
            }
            Err( e ) => tx.send( Some( e ) ),
          }
    } );
    if let Some( error ) = rx.recv().unwrap()
    {
      panic!( "Failed to run a DBus interface: {error}" );
    }

    Ok
    ( 
      Self
      {
        active_window,
        _kwin_script : kwin_script,
      } 
    )
  }
}

impl ActiveWindowManager for KdeWindowManager
{
  fn get_active_window_data( &mut self ) -> Result< ActiveWindowData >
  {
    let data = self.active_window.lock().unwrap();
    Ok( 
      ActiveWindowData
      {
        window_title : data.caption.clone().into(),
        process_name : None,
        app_id : Some(data.resource_name.clone().into()),
      } 
    )
  }
}
