use std::path::PathBuf;

use gtk::{
  gio::{AppInfo, DesktopAppInfo},
  glib::GString,
  prelude::*,
};
use log::debug;
use regex::Regex;

use crate::{entry::app_entry::AppEntry, launcher::util::icon::load_icon};

pub struct App;

impl App {
  pub fn all() -> Vec<AppEntry> {
    debug!("Reading apps");
    let mut results = Vec::new();

    let re = Regex::new(r"%[uUfFdDnNickvm]").unwrap();

    for a in AppInfo::all() {
      if !a.should_show() {
        continue;
      }

      if let Some(exec) = a.commandline() {
        let icon = if a.icon().is_none() {
          None
        } else {
          let st = gtk::prelude::IconExt::to_string(&a.icon().unwrap())
            .unwrap()
            .to_string();

          Some(load_icon(&st, 40))
        };

        if let Some(file) = a.id() {
          let exec: Vec<String> =
            shell_words::split(&*re.replace(&*exec.display().to_string(), "")).unwrap();

          let terminal = if let Some(desktop) = DesktopAppInfo::new(&a.id().unwrap()) {
            desktop.boolean("Terminal")
          } else {
            false
          };

          results.push(AppEntry {
            name: a.display_name().to_string(),
            description: a
              .description()
              .unwrap_or_else(|| GString::from(""))
              .to_string(),
            file: PathBuf::from(file.to_string()),
            icon,
            exec,
            terminal,
          })
        }
      }
    }

    debug!("Read {} apps", results.len());

    results
  }
}
