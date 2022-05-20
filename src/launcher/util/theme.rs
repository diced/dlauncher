use std::{
  fs::{read, read_to_string, write},
  path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::launcher::util::config::Config;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeJson {
  pub manifest_version: String,
  pub name: String,
  pub display_name: String,
  pub extend_theme: Option<String>,
  pub css_file: String,
  #[serde(rename = "css_file_gtk_3.20+")]
  pub css_file_gtk_3_20: Option<String>,
  pub matched_text_hl_colors: MatchedTextHlColors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchedTextHlColors {
  pub when_selected: String,
  pub when_not_selected: String,
}

pub struct Theme {
  pub inner: ThemeJson,
  config: Config,
}

impl Theme {
  pub fn new(config: Config, bytes: &[u8]) -> Self {
    let inner = serde_json::from_slice(bytes).unwrap();

    Self { inner, config }
  }

  pub fn read_file(&self) -> String {
    read_to_string(self.path().join(&self.inner.css_file)).unwrap()
  }

  pub fn path(&self) -> PathBuf {
    self.config.dir().join("themes").join(&self.inner.name)
  }

  pub fn css_file(&self) -> &str {
    self
      .inner
      .css_file_gtk_3_20
      .as_ref()
      .unwrap_or(&self.inner.css_file)
  }

  pub fn compile_css(&self) -> PathBuf {
    let css_file = self.path().join(&self.css_file());

    if let Some(extend_theme) = &self.inner.extend_theme {
      let generated_css = self.path().join("generated.css");

      let extend_bytes = &read(
        self
          .config
          .dir()
          .join("themes")
          .join(extend_theme)
          .join("manifest.json"),
      )
      .unwrap();
      let extend_theme: Theme = Theme::new(self.config.clone(), extend_bytes);

      let st_ = format!(
        "@import url({:?});\n\n{}",
        extend_theme.compile_css(),
        self.read_file(),
      );
      write(&generated_css, st_).unwrap();

      generated_css
    } else {
      css_file
    }
  }
}
