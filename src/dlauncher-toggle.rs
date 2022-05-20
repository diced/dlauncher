use std::time::Duration;

use dbus::blocking::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let conn = Connection::new_session()?;
  let proxy = conn.with_proxy("com.dlauncher.server", "/open", Duration::from_millis(5000));
  proxy.method_call("com.dlauncher.server", "OpenWindow", ())?;

  Ok(())
}
