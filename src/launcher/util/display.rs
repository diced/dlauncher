use gtk::gdk::{Display, Monitor, prelude::*};
use gtk::gio::Settings;

pub fn monitor() -> Monitor {
  let display = Display::default().unwrap();
  let seat = display.default_seat().unwrap();
  let (_, x, y) = seat.pointer().unwrap().position();
  if let Some(monitor) = display.monitor_at_point(x, y) {
      monitor
  } else if let Some(monitor) = display.primary_monitor() {
    monitor
  } else if let Some(monitor) = display.monitor(0) {
    monitor
  } else {
      panic!("Couldn't get monitor through various methods...")
  }
}

pub fn scaling_factor() -> f32 {
  let monitor_scaling = monitor().scale_factor();
  let text_scaling = Settings::new("org.gnome.desktop.interface");
  let text_scaling = text_scaling.double("text-scaling-factor");

  (monitor_scaling as f64 * text_scaling) as f32
}
