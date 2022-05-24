use std::{fmt, rc::Rc};

use crate::{
  entry::{extension_entry::ExtensionEntry, ResultEntry},
  extension::ExtensionContext,
  fuzzy::MatchingBlocks,
  launcher::{result::ResultWidget, window::Window},
  util::no_match,
};

#[derive(Debug, Clone)]
/// An extension response, that can create a ResultWidget.
pub struct ExtensionResponse {
  extension_name: String,
  pub lines: Vec<ExtensionResponseLine>,
}

pub type OnEnterFn = Rc<Option<Box<dyn Fn(ExtensionContext)>>>;
/// A line consisting of a name, description, icon, and an optional on_enter function and match vector.
pub struct ExtensionResponseLine {
  pub name: String,
  pub description: String,
  pub icon: ExtensionResponseIcon,
  pub match_: MatchingBlocks,
  pub on_enter: OnEnterFn,
}

#[derive(Debug, Clone)]
/// Represents an icon, which can be a themed-icon or an svg string.
pub struct ExtensionResponseIcon {
  pub type_: ExtensionResponseIconType,
  pub value: String,
}

#[derive(Debug, Clone)]
pub enum ExtensionResponseIconType {
  ThemedIcon,
  SVGStringIcon,
}

impl ExtensionResponse {
  /// Initialize a ExtensionResponse Builder, that takes in the extension name.
  /// The extension_name **must** be the same as the name returned inside of
  /// [ExtensionContext](ExtensionContext)
  pub fn builder(extension_name: impl Into<String>) -> Self {
    Self {
      extension_name: extension_name.into(),
      lines: vec![],
    }
  }

  /// Create a line, that returns ExtensionResponse. This function does not take in a match or on_enter.
  pub fn line(
    &mut self,
    name: impl Into<String>,
    description: impl Into<String>,
    icon: ExtensionResponseIcon,
  ) -> &mut Self {
    self.lines.push(ExtensionResponseLine {
      name: name.into(),
      description: description.into(),
      icon,
      match_: (vec![], 0),
      on_enter: Rc::new(None),
    });

    self
  }

  /// Add a line with a match. The match must be a [Vec](Vec)<[usize](usize)>,
  /// usually you can get the vector from the [matches](../../util/fn.matches.html) function
  pub fn line_match(
    &mut self,
    name: impl Into<String>,
    description: impl Into<String>,
    icon: ExtensionResponseIcon,
    match_: MatchingBlocks,
  ) -> &mut Self {
    self.lines.push(ExtensionResponseLine {
      name: name.into(),
      description: description.into(),
      icon,
      match_,
      on_enter: Rc::new(None),
    });

    self
  }

  pub fn line_on_enter<F>(
    &mut self,
    name: impl Into<String>,
    description: impl Into<String>,
    icon: ExtensionResponseIcon,
    on_enter: F,
  ) -> &mut Self
  where
    F: Fn(ExtensionContext) + 'static,
  {
    self.lines.push(ExtensionResponseLine {
      name: name.into(),
      description: description.into(),
      icon,
      match_: no_match(),
      on_enter: Rc::new(Some(Box::new(on_enter))),
    });

    self
  }

  pub fn line_match_on_enter<F>(
    &mut self,
    name: impl Into<String>,
    description: impl Into<String>,
    icon: ExtensionResponseIcon,
    match_: MatchingBlocks,
    on_enter: F,
  ) -> &mut Self
  where
    F: Fn(ExtensionContext) + 'static,
  {
    self.lines.push(ExtensionResponseLine {
      name: name.into(),
      description: description.into(),
      icon,
      match_,
      on_enter: Rc::new(Some(Box::new(on_enter))),
    });

    self
  }

  pub fn build(&self, window: Window) -> Vec<ResultWidget> {
    let mut result = Vec::new();

    for line in &self.lines {
      let entry = ResultEntry::Extension(ExtensionEntry::new(&self.extension_name, line.clone()));
      result.push(ResultWidget::new(
        entry.clone(),
        window.clone(),
        line.match_.clone(),
      ));
    }

    result
  }

  pub fn build_and_show(&self, window: Window, override_: bool) {
    let result = self.build(window.clone());
    window.show_results(result, override_);
  }
}

impl ExtensionResponseLine {
  pub fn builder() -> Self {
    Self {
      name: String::new(),
      description: String::new(),
      match_: no_match(),
      icon: ExtensionResponseIcon::themed(""),
      on_enter: Rc::new(None),
    }
  }

  pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
    self.name = name.into();
    self
  }

  pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
    self.description = description.into();
    self
  }

  pub fn icon(&mut self, icon: ExtensionResponseIcon) -> &mut Self {
    self.icon = icon;
    self
  }

  pub fn match_(&mut self, match_: MatchingBlocks) -> &mut Self {
    self.match_ = match_;
    self
  }

  pub fn on_enter(&mut self, on_enter: fn(ExtensionContext) -> ()) -> &mut Self {
    self.on_enter = Rc::new(Some(Box::new(on_enter)));
    self
  }
}

impl ExtensionResponseIcon {
  pub fn themed(name: impl Into<String>) -> Self {
    Self {
      type_: ExtensionResponseIconType::ThemedIcon,
      value: name.into(),
    }
  }

  pub fn svg(svg_string: impl Into<String>) -> Self {
    Self {
      type_: ExtensionResponseIconType::SVGStringIcon,
      value: svg_string.into(),
    }
  }
}

impl fmt::Debug for ExtensionResponseLine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ExtensionResponseLine")
      .field("name", &self.name)
      .field("description", &self.description)
      .field("icon", &self.icon)
      .field("match_", &self.match_)
      .finish()
  }
}

impl Clone for ExtensionResponseLine {
  fn clone(&self) -> Self {
    Self {
      name: self.name.clone(),
      description: self.description.clone(),
      match_: self.match_.clone(),
      icon: self.icon.clone(),
      on_enter: self.on_enter.clone(),
    }
  }
}
