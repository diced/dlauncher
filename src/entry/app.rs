use std::{
  fs::{write, File},
  io::{BufRead, BufReader},
  path::PathBuf,
  sync::{Arc, Mutex},
};

use gtk::{gdk_pixbuf::Pixbuf, glib::GString, prelude::*, gio::DesktopAppInfo};
use log::debug;
use regex::Regex;

use crate::{
  entry::ResultEntry,
  launcher::{
    result::ResultWidget,
    util::icon::{default_pixbuf, load_icon},
    window::Window,
  },
  util::{launch_detached, no_match},
};

#[derive(Debug, Clone)]
pub struct App {
  pub name: String,
  pub desc: String,
  pub file: PathBuf,
  pub icon: Option<Pixbuf>,
  pub exec: Vec<String>,
  pub terminal: bool,
}

#[derive(Debug, Clone)]
pub struct Recent {
  pub num: u32,
  pub file: PathBuf,
}

impl App {
  pub fn read_all() -> Vec<Self> {
    debug!("Reading apps");
    let mut results = Vec::new();

    let re = Regex::new(r"%[uUfFdDnNickvm]").unwrap();

    for a in gtk::gio::AppInfo::all() {
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

          results.push(Self {
            name: a.display_name().to_string(),
            desc: a
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

  pub fn get_recents(path: PathBuf) -> Vec<Recent> {
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

  pub fn execute(&self, window: Window) {
    let spawn_args = if self.terminal && window.config.launcher.terminal_command.as_ref().is_some() {
      let cmd = window.config.launcher.terminal_command.as_ref().unwrap();
      let full = shell_words::join(&self.exec);
      let cmd = cmd.replace("{}", &full);
      
      shell_words::split(&cmd)
        .unwrap()
    } else {
      self.exec.clone()
    };

    debug!("Attempting to launch {:?}", spawn_args);

    launch_detached(spawn_args, vec![]);

    let mut recents = window.state.recents.lock().unwrap();
    let recent = recents.iter_mut().find(|r| r.file == self.file);
    if let Some(recent) = recent {
      recent.num += 1;
    } else {
      recents.push(Recent {
        num: 1,
        file: self.file.clone(),
      });
    }
    recents.sort_by(|a, b| b.num.cmp(&a.num));
    Self::recents_to_file(recents.to_vec(), &window.config.recents());
  }

  pub fn icon(&self) -> Pixbuf {
    match &self.icon {
      Some(icon) => icon.clone(),
      None => default_pixbuf(40),
    }
  }
}

impl Recent {
  pub fn to_result(&self, window: Window, apps: Arc<Mutex<Vec<App>>>) -> Option<ResultWidget> {
    let apps = apps.lock().unwrap();
    let app = apps.iter().find(|app| app.file == self.file);
    app.map(|app| ResultWidget::new(ResultEntry::App(app.clone()), window, no_match()))
  }
}
