use anyhow::{anyhow, Result};
use async_trait::async_trait;
use objc2::{rc::Retained, runtime::AnyObject, AnyThread};
use objc2_foundation::{ns_string, NSAppleScript, NSDictionary, NSString};

use crate::window_api::process_name;

use super::{ActiveWindowData, ActiveWindowManager};

const SOURCE: &str = r#"
global frontApp, frontAppName, windowTitle

set windowTitle to ""
tell application "System Events"
    set frontApp to first application process whose frontmost is true
    set frontAppName to name of frontApp
    set processID to unix id of frontApp -- Get the process ID

    tell process frontAppName
        try
            tell (1st window whose value of attribute "AXMain" is true)
                set windowTitle to value of attribute "AXTitle"
            end tell
        end try
    end tell
end tell

return frontAppName & "
" & windowTitle & "
" & processID
"#;

#[derive(Default)]
pub struct MacosManger {}

impl MacosManger {
  pub fn new() -> Self {
    Default::default()
  }
}

#[async_trait]
impl ActiveWindowManager for MacosManger {
  async fn get_active_window_data(&mut self) -> Result<ActiveWindowData> {
    let hello = NSAppleScript::alloc();
    let help = unsafe { NSAppleScript::initWithSource(hello, ns_string!(SOURCE)) }
      .expect("Source script is expected to be always correct");
    let mut err: Option<Retained<NSDictionary<NSString, AnyObject>>> = None;
    let descriptor = unsafe { help.executeAndReturnError(Some(&mut err)) };

    let data = unsafe { descriptor.stringValue() }
      .ok_or_else(|| anyhow!("Source script is expected to return string value but got {:?}", descriptor))?
      .to_string();
    let values = data.lines().collect::<Vec<_>>();
    let app_id = values[0];
    let window_title = values[1];
    let process_id = values[2].parse::<u32>()?;
    let process_name = process_name::get_process_name(process_id)?;

    return Ok(ActiveWindowData {
      window_title: window_title.to_string().into(),
      process_name: process_name.map(Into::into),
      app_id: Some(app_id.to_string().into()),
    });
  }
}
