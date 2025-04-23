use anyhow::Result;
use sysinfo::Pid;


#[allow(unused)]
pub(crate) fn get_process_name( id : u32 ) -> Result< Option< String > >
{
  let system = sysinfo::System::new_all();
  let Some( process ) = system.process( Pid::from_u32( id ) )
  else
  {
    return Ok( None );
  };

  Ok( process.exe().and_then( | v | v.to_str() ).map( | v | v.to_string() ) )
}
