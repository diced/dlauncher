use gtk::{
  gdk_pixbuf::{Pixbuf, PixbufLoader},
  prelude::*,
};
use std::fmt;

use crate::{
  extension::response::{
    ExtensionResponseIcon, ExtensionResponseIconType, ExtensionResponseLine, OnEnterFn,
  },
  launcher::util::icon::load_icon,
};

pub struct Ext {
  pub extension_name: String,
  pub name: String,
  pub description: String,
  pub icon: ExtensionResponseIcon,
  pub on_enter: OnEnterFn,
}

impl Ext {
  pub fn new(extension_name: &str, line: ExtensionResponseLine) -> Self {
    Self {
      extension_name: extension_name.to_string(),
      name: line.name,
      description: line.description,
      icon: line.icon,
      on_enter: line.on_enter,
    }
  }

  pub fn icon(&self) -> Pixbuf {
    match self.icon.type_ {
      ExtensionResponseIconType::ThemedIcon => load_icon(&self.icon.value, 40),
      ExtensionResponseIconType::SVGStringIcon => {
        let loader = PixbufLoader::new();
        loader.write(self.icon.value.as_bytes()).unwrap();
        loader.close().unwrap();

        loader.pixbuf().unwrap()
      }
    }
  }
}

impl fmt::Debug for Ext {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Ext")
      .field("extension_name", &self.extension_name)
      .field("name", &self.name)
      .field("description", &self.description)
      .field("icon", &self.icon)
      .finish()
  }
}

impl Clone for Ext {
  fn clone(&self) -> Self {
    Self {
      extension_name: self.extension_name.clone(),
      name: self.name.clone(),
      description: self.description.clone(),
      icon: self.icon.clone(),
      on_enter: self.on_enter.clone(),
    }
  }
}
