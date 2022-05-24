use std::path::PathBuf;

use gtk::gdk_pixbuf::Pixbuf;
use log::debug;

use crate::{
  launcher::{
    util::{icon::default_pixbuf, recent::Recent},
    window::Window,
  },
  util::launch_detached,
};

#[derive(Debug, Clone)]
pub struct AppEntry {
  pub name: String,
  pub description: String,
  pub file: PathBuf,
  pub icon: Option<Pixbuf>,
  pub exec: Vec<String>,
  pub terminal: bool,
}

impl AppEntry {
  pub fn execute(&self, window: Window) {
    let spawn_args = if self.terminal && window.config.launcher.terminal_command.as_ref().is_some()
    {
      let cmd = window.config.launcher.terminal_command.as_ref().unwrap();
      let full = shell_words::join(&self.exec);
      let cmd = cmd.replace("{}", &full);

      shell_words::split(&cmd).unwrap()
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
    Recent::recents_to_file(recents.to_vec(), &window.config.recents());
  }

  pub fn icon(&self) -> Pixbuf {
    match &self.icon {
      Some(icon) => icon.clone(),
      None => default_pixbuf(40),
    }
  }
}
