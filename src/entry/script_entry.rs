use gtk::{
  gdk_pixbuf::{Pixbuf, PixbufLoader},
  prelude::*,
};

use crate::{
  launcher::util::icon::load_icon,
  script::{Script, ScriptIcon},
};

#[derive(Debug, Clone)]
pub struct ScriptEntry {
  script: Script,
}

impl ScriptEntry {
  pub fn new(script: Script) -> Self {
    Self { script }
  }

  pub fn name(&self) -> &str {
    &self.script.meta.name
  }

  pub fn desc(&self) -> &str {
    &self.script.meta.desc
  }

  pub fn run(&self) -> () {
    self.script.run();
  }

  pub fn icon(&self) -> Pixbuf {
    match &self.script.meta.icon {
      ScriptIcon::Themed(value) => load_icon(&value, 40),
      ScriptIcon::Svg(value) => {
        let loader = PixbufLoader::new();
        loader.write(value.as_bytes()).unwrap();
        loader.close().unwrap();

        loader.pixbuf().unwrap()
      }
    }
  }
}
