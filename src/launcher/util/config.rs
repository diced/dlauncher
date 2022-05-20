use std::{
  fs::{create_dir_all, read, write},
  path::PathBuf,
};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{
  extension::{Extension, ExtensionExitCode},
  launcher::{util::theme::Theme, window::Window},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  /// Main configuration
  pub main: ConfigMain,
  /// Launcher options
  pub launcher: ConfigLauncher,
  /// Keybinds used when navigating through results in the launcher
  /// These keybinds are not for opening/toggling the launcher.
  pub keybinds: Option<ConfigKeybinds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMain {
  /// Whether to run the launcher in the background.
  /// When running in a daemon, you can toggle the window to appear by running `dlauncher-toggle`.
  ///
  ///    If using a window manager or a desktop environment you can use its specific hot key manager and bind a keypress to `dlauncher-toggle`.
  pub daemon: bool,
  /// Least score for matches made to the query and app name
  pub least_score: usize,
  /// Extensions (string is the name of the extension)
  /// Extensions are located in `($XDG_CONFIG_HOME or ~/.config)/dlauncher/extensions`
  ///
  /// For more information on how extensions work see [Extensions](../../../extension/index.html)
  /// ```toml
  /// extensions = ["zero_width_space.so"]
  /// ```
  pub extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigLauncher {
  /// Theme for the window
  /// Themes are located at `($XDG_CONFIG_HOME or ~/.config)/dlauncher/themes/`
  pub color_theme: String,
  /// Number of frequent apps to show with no query
  pub frequent_apps: u16,
  /// Clear input whenever the window is shown (daemon mode)
  pub clear_input: bool,
  /// Hide when the mouse loses focus on the window
  pub hide_on_focus_lost: bool,
  /// Run application thorugh a terminal if a desktop entry has `Terminal=true`. `{}` will be replaced with the application's command.
  ///
  /// Examples:
  /// ```toml
  /// terminal = 'alacritty -e "{}"'
  /// terminal = 'xterm -c "{}"'
  /// ```
  pub terminal_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigKeybinds {
  pub result_up: Option<String>,
  pub result_down: Option<String>,
  pub close: Option<String>,
  pub open: Option<String>,
}

pub struct Keybinds {
  pub result_up: String,
  pub result_down: String,
  pub close: String,
  pub open: String,
}

impl Config {
  pub fn default() -> Self {
    Self {
      main: ConfigMain {
        daemon: true,
        least_score: 60,
        extensions: vec![],
      },
      launcher: ConfigLauncher {
        color_theme: "light".to_string(),
        frequent_apps: 6,
        clear_input: true,
        hide_on_focus_lost: false,
        terminal_command: None,
      },
      keybinds: None,
    }
  }

  pub fn read() -> Self {
    let home = std::env::var("HOME").expect("you are homeless");
    let config_path = PathBuf::from(home).join(".config/dlauncher");
    let abs_config_path = config_path.join("dlauncher.toml");

    let mut first = false;
    let theme = if !abs_config_path.exists() {
      first = true;
      create_dir_all(&config_path).unwrap();

      let default = Self::default();
      let bytes = toml::to_vec(&default).unwrap();
      write(&abs_config_path, bytes).unwrap();

      info!(
        "There was no config file found. A default config file has been created at {:#?}",
        abs_config_path
      );

      default
    } else {
      let bytes = read(abs_config_path).unwrap();
      toml::from_slice(&bytes).unwrap()
    };

    if first {
      theme.setup();
    }

    theme
  }

  pub fn keybinds(&self) -> Keybinds {
    let k = self.keybinds.as_ref().unwrap_or(&ConfigKeybinds {
      result_up: None,
      result_down: None,
      close: None,
      open: None,
    });

    Keybinds {
      result_up: k
        .result_up
        .as_ref()
        .unwrap_or(&"Up".to_string())
        .to_string(),
      result_down: k
        .result_down
        .as_ref()
        .unwrap_or(&"Down".to_string())
        .to_string(),
      close: k
        .close
        .as_ref()
        .unwrap_or(&"Escape".to_string())
        .to_string(),
      open: k.open.as_ref().unwrap_or(&"Return".to_string()).to_string(),
    }
  }

  pub fn dir(&self) -> PathBuf {
    PathBuf::from(std::env::var("HOME").expect("you are homeless")).join(".config/dlauncher")
  }

  pub fn themes_dir(&self) -> PathBuf {
    self.dir().join("themes")
  }

  pub fn setup(&self) {
    let theme_path = self.themes_dir().join("light");
    create_dir_all(&theme_path).unwrap();


    // Copies over default theme.
    let default_manifest = include_bytes!("../../../data/themes/light/manifest.json");
    let default_themecss = include_bytes!("../../../data/themes/light/theme.css");
    let default_themegtk = include_bytes!("../../../data/themes/light/theme-gtk-3.20.css");
    let default_resetcss = include_bytes!("../../../data/themes/light/reset.css");

    write(&theme_path.join("manifest.json"), default_manifest).unwrap();
    write(&theme_path.join("theme.css"), default_themecss).unwrap();
    write(&theme_path.join("theme-gtk-3.20.css"), default_themegtk).unwrap();
    write(&theme_path.join("reset.css"), default_resetcss).unwrap();
  }

  pub fn recents(&self) -> PathBuf {
    self.dir().join("dlauncher.druncache")
  }

  pub fn theme(&self) -> Theme {
    let theme = read(
      self
        .themes_dir()
        .join(&self.launcher.color_theme)
        .join("manifest.json"),
    )
    .unwrap();

    Theme::new(self.clone(), &theme)
  }

  pub fn extensions(&self, window: &Window) -> Vec<Extension> {
    let exts = self
      .main
      .extensions
      .iter()
      .map(|ext| Extension::new(window.clone(), self.clone(), ext.clone()))
      .collect::<Vec<Extension>>();

    for ext in &exts {
      debug!("Starting extension {}", ext.name);
      match ext.on_init() {
        ExtensionExitCode::Ok => info!("Started extension {}", ext.name),
        ExtensionExitCode::Error(err) => {
          error!("[{}] An error occurred on `on_init`: {}", ext.name, err)
        }
      }
    }

    exts
  }
}
