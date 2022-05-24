use std::{
  fs::{write, File},
  io::{BufRead, BufReader},
  path::PathBuf,
  sync::{Arc, Mutex},
};

use log::debug;

use crate::{
  entry::{app_entry::AppEntry, ResultEntry},
  launcher::{result::ResultWidget, window::Window},
  util::no_match,
};

#[derive(Debug, Clone)]
pub struct Recent {
  pub num: u32,
  pub file: PathBuf,
}

impl Recent {
  pub fn all(path: PathBuf) -> Vec<Recent> {
    debug!("Fetching recent apps");
    let file = File::open(&path);
    let mut recents: Vec<Recent> = vec![];
    if let Ok(file) = file {
      let lines = BufReader::new(&file).lines();
      for line in lines {
        let line = line.unwrap();
        let line = line.trim().split_once(' ').unwrap();

        recents.push(Recent {
          num: line.0.parse::<u32>().expect("Failed to parse number"),
          file: PathBuf::from(line.1),
        });
      }
      file.sync_all().unwrap();
    } else {
      write(&path, "").unwrap();
    }
    debug!("Recent apps refreshed");

    recents.sort_by(|a, b| b.num.cmp(&a.num));

    recents
  }

  pub fn recents_to_file(recents: Vec<Recent>, path: &PathBuf) {
    let st = recents
      .iter()
      .map(|r| format!("{} {}", r.num, r.file.to_str().unwrap()))
      .collect::<Vec<String>>()
      .join("\n");
    write(&path, st).unwrap();
  }

  pub fn to_result(&self, window: Window, apps: Arc<Mutex<Vec<AppEntry>>>) -> Option<ResultWidget> {
    let apps = apps.lock().unwrap();
    let app = apps.iter().find(|app| app.file == self.file);
    app.map(|app| ResultWidget::new(ResultEntry::App(app.clone()), window, no_match()))
  }
}
