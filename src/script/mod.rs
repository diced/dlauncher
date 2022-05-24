use std::{
  fs::{create_dir_all, read_dir, read_to_string},
  path::PathBuf,
};

use log::error;

use crate::{launcher::util::config::Config, util::launch_detached};

#[derive(Debug, Clone)]
pub struct ScriptMeta {
  pub name: String,
  pub desc: String,
  pub icon: ScriptIcon,
}

#[derive(Debug, Clone)]
pub enum ScriptIcon {
  Themed(String),
  Svg(String),
}

#[derive(Debug, Clone)]
pub struct Script {
  pub meta: ScriptMeta,
  pub path: PathBuf,
}

impl ScriptMeta {
  pub fn new(contents: String) -> Self {
    let lines = contents.lines();
    let mut meta = ScriptMeta {
      name: "".to_string(),
      desc: "".to_string(),
      icon: ScriptIcon::Themed("".to_string()),
    };

    // probably some other way that is faster than this lol
    for line in lines {
      if line.starts_with("# Name") {
        let name = line.split("# Name ").last().unwrap();
        meta.name = name.trim().to_string();
      } else if line.starts_with("# Desc") {
        let desc = line.split("# Desc ").last().unwrap();
        meta.desc = desc.trim().to_string();
      } else if line.starts_with("# Icon-svg") {
        let icon = line.split("# Icon-svg ").last().unwrap();
        meta.icon = ScriptIcon::Svg(icon.to_string());
      } else if line.starts_with("# Icon-themed") {
        let icon = line.trim().split("# Icon-themed ").last().unwrap();
        meta.icon = ScriptIcon::Themed(icon.to_string());
      }
    }

    meta
  }
}

impl Script {
  pub fn all(config: &Config) -> Vec<Self> {
    let scripts_dir = config.dir().join("scripts");
    create_dir_all(&scripts_dir).unwrap();

    let mut scripts = Vec::new();

    for dirent in read_dir(&scripts_dir).unwrap() {
      let dirent = dirent.unwrap();
      let sc = Script::new(dirent.path());
      if sc.meta.name.is_empty() {
        error!(
          "Script {} has no name, skipping this script...",
          sc.path.display()
        );
      } else {
        scripts.push(sc);
      }
    }

    scripts
  }

  pub fn new(path: PathBuf) -> Self {
    let contents = read_to_string(&path).unwrap();
    let meta = ScriptMeta::new(contents);
    Script { path, meta }
  }

  pub fn run(&self) {
    launch_detached(vec!["sh", "-c", self.path.to_str().unwrap()], vec![]);
  }
}
