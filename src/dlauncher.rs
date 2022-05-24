use dbus::blocking::Connection;
use dbus_crossroads::{Context, Crossroads, IfaceBuilder};
use gtk::{
  glib,
  glib::{Receiver, Sender},
  prelude::*,
};
use log::{debug, info};

use dlauncher::{
  launcher::{util::config::Config, window::Window},
  util::init_logger,
};

fn main() {
  init_logger();
  debug!("Starting dlauncher...");

  let config = Config::read();
  let application = gtk::Application::new(Some("net.launchpad.dlauncher"), Default::default());

  application.connect_activate(move |application| {
    let windows = Window::new(application, &config);
    windows.build_ui();
    info!("Started dlauncher");

    if !config.main.daemon {
      // skip initializing a dbus interface if daemon isn't enabled & show window
      windows.window.show_all();
      info!("Running in non-daemon mode");
    } else {
      debug!("Starting dbus interface");
      let (tx, rx): (Sender<bool>, Receiver<bool>) =
        glib::MainContext::channel(glib::PRIORITY_DEFAULT);

      std::thread::spawn(move || {
        let c = Connection::new_session().unwrap();
        c.request_name("com.dlauncher.server", false, true, false)
          .unwrap();
        let mut cr = Crossroads::new();

        let iface_token = cr.register(
          "com.dlauncher.server",
          |b: &mut IfaceBuilder<Sender<bool>>| {
            b.method(
              "OpenWindow",
              (),
              (),
              move |_: &mut Context, thread_tx, (): ()| {
                thread_tx.send(true).unwrap();
                Ok(())
              },
            );
          },
        );

        cr.insert("/open", &[iface_token], tx);
        cr.serve(&c).unwrap();
      });

      rx.attach(None, move |msg| {
        if msg {
          debug!("Received message from dbus interface, showing window.");
          windows.show_window();
        }

        Continue(true)
      });
    };
  });

  application.run();
}
