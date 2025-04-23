use anyhow::{anyhow, Result};
use async_trait::async_trait;
use objc2::{rc::Retained, runtime::AnyObject, AnyThread};
use objc2_foundation::{ns_string, NSAppleScript, NSDictionary, NSString};

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
    unsafe {
      let help = NSAppleScript::initWithSource(hello, ns_string!(SOURCE)).expect("Source script is expected to be always correct");
      let mut err: Option<Retained<NSDictionary<NSString, AnyObject>>> = None;
      let descriptor = help.executeAndReturnError(Some(&mut err));
      dbg!(descriptor.stringValue());
      println!("{}", descriptor.stringValue().unwrap());

    }

    Err(anyhow!("Still developing"))
  }
}
