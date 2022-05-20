#![doc = include_str!("../../EXTENSIONS.md")]

use std::sync::Arc;

use libloading::{Library, Symbol};
use log::debug;

use crate::{
  extension::{config::ExtensionConfig, query::Query},
  launcher::{util::config::Config, window::Window},
};

pub mod config;
pub mod query;
pub mod response;

/// Function signature used for native extensions
pub type ExtensionOutputFunc = unsafe extern "C" fn(ExtensionContext) -> ExtensionExitCode;

/// Return codes for native extensions
pub enum ExtensionExitCode {
  /// When the extension returns successfully
  Ok,
  /// If the extension encounters an error, it should return this with an explanation.
  ///
  /// ```
  /// use dlauncher::extension::ExtensionExitCode;
  /// ExtensionExitCode::Error("Failed to do something");
  /// ```
  Error(&'static str),
}

#[derive(Debug, Clone)]
pub struct Extension {
  /// The shared object library
  pub library: Arc<Library>,
  /// Copy of Dlauncher window for use in the extension
  pub window: Window,
  /// Copy of Dlauncher config for use in the extension
  pub config: ExtensionConfig,
  /// The extension's name, used for identification and more to keep extensions in line.
  pub name: String,
}

#[derive(Debug, Clone)]
pub struct ExtensionContext {
  /// Extensions name
  ///
  /// Used for building responses
  pub name: String,
  /// The Dlauncher window, useful when directly interfacing with Dlauncher which allows for great flexibility.
  pub window: Window,
  /// Input when called through on_input(), this will be `None` if called through other functions that don't require an input.
  pub input: Option<Query>,
  /// The Dlauncher main configuration
  pub config: ExtensionConfig,
}

impl Extension {
  pub fn new(window: Window, config: Config, extension_name: String) -> Extension {
    unsafe {
      let filename = config.dir().join("extensions").join(&extension_name);
      let library = Library::new(filename).unwrap();

      Extension {
        library: Arc::new(library),
        window,
        config: ExtensionConfig::new(&config, &extension_name),
        name: extension_name,
      }
    }
  }

  /// on_input is called when a user types something into the input.
  pub fn on_input(&self, input: &str) -> ExtensionExitCode {
    unsafe {
      let output: Symbol<ExtensionOutputFunc> = self.library.get(b"on_input").unwrap();

      output(ExtensionContext {
        name: self.name.clone(),
        input: Some(Query::from_str(input)),
        window: self.window.clone(),
        config: self.config.clone(),
      })
    }
  }

  /// on_init is called when dlauncher is starting.
  pub fn on_init(&self) -> ExtensionExitCode {
    unsafe {
      if let Ok(output) = self.library.get::<ExtensionOutputFunc>(b"on_init") {
        output(ExtensionContext {
          name: self.name.clone(),
          input: None,
          window: self.window.clone(),
          config: self.config.clone(),
        })
      } else {
        debug!("Extension {} has no on_init function, skipped", self.name);
        ExtensionExitCode::Ok
      }
    }
  }

  /// on_open is called when the window is toggled open.
  pub fn on_open(&self) -> ExtensionExitCode {
    unsafe {
      if let Ok(output) = self.library.get::<ExtensionOutputFunc>(b"on_open") {
        output(ExtensionContext {
          name: self.name.clone(),
          input: None,
          window: self.window.clone(),
          config: self.config.clone(),
        })
      } else {
        debug!("Extension {} has no on_open", self.name);
        ExtensionExitCode::Ok
      }
    }
  }
}
