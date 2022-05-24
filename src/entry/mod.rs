use gtk::gdk_pixbuf::Pixbuf;

use crate::{
  extension::{config::ExtensionConfig, ExtensionContext},
  launcher::{
    util::{config::Config, icon::default_pixbuf},
    window::Window,
  },
};

pub mod app_entry;
pub mod extension_entry;
pub mod script_entry;

#[derive(Debug, Clone)]
pub enum ResultEntry {
  App(app_entry::AppEntry),
  Extension(extension_entry::ExtensionEntry),
  Script(script_entry::ScriptEntry),
  None,
}

impl ResultEntry {
  pub fn name(&self) -> &str {
    match self {
      ResultEntry::App(app) => &app.name,
      ResultEntry::Extension(ext) => &ext.name,
      ResultEntry::Script(script) => script.name(),
      ResultEntry::None => "No results",
    }
  }

  pub fn description(&self) -> &str {
    match self {
      ResultEntry::App(app) => &app.description,
      ResultEntry::Extension(ext) => &ext.description,
      ResultEntry::Script(script) => script.desc(),
      ResultEntry::None => "No results found.",
    }
  }

  pub fn icon(&self) -> Pixbuf {
    match self {
      ResultEntry::App(app) => app.icon(),
      ResultEntry::Extension(ext) => ext.icon(),
      ResultEntry::Script(script) => script.icon(),
      ResultEntry::None => default_pixbuf(40),
    }
  }

  pub fn execute(&self, window: Window) {
    match self {
      ResultEntry::App(app) => app.execute(window),
      ResultEntry::Extension(ext) => {
        if let Some(on_enter) = ext.on_enter.as_ref() {
          let config = Config::read();
          on_enter(ExtensionContext {
            name: ext.extension_name.clone(),
            window,
            input: None,
            config: ExtensionConfig::new(&config, &ext.extension_name),
          })
        }
      }
      ResultEntry::Script(script) => script.run(),
      ResultEntry::None => (),
    }
  }
}
