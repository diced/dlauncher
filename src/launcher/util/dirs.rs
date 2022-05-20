use std::{env, path::PathBuf};

pub struct Dirs {
  pub home: PathBuf,
  pub config: PathBuf,
  pub extensions: PathBuf,
  pub extension_configs: PathBuf,
  pub themes: PathBuf,
}

impl Dirs {
  pub fn new() -> Dirs {
    let home = env::var("HOME").expect("homeless");
    let home = PathBuf::from(home);
    let config = home.join(".config/dlauncher");
    let extensions = config.join("extensions");
    let extension_configs = extensions.join("extension_config");
    let themes = config.join("themes");

    Dirs {
      home,
      config,
      extensions,
      extension_configs,
      themes,
    }
  }

  pub fn create_dirs(&self) {
    let dirs = [
      &self.config,
      &self.extensions,
      &self.extension_configs,
      &self.themes,
    ];

    for dir in dirs.iter() {
      if !dir.exists() {
        std::fs::create_dir_all(dir).expect("failed to create dirs");
      }
    }
  }
}
