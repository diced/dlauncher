use gtk::{prelude::*, Builder, EventBox, Image, Label};

use crate::{entry::ResultEntry, launcher::window::Window, fuzzy::MatchingBlocks};

#[derive(Debug, Clone)]
pub struct ResultWidget {
  pub builder: Builder,
  pub selected: bool,
  pub match_: MatchingBlocks,
  pub entry: ResultEntry,
  pub index: u16,
  pub window: Window,
}

impl ResultWidget {
  pub fn new(entry: ResultEntry, window: Window, match_: MatchingBlocks) -> Self {
    let result_str = include_str!("../../data/ui/result.ui");
    let builder = Builder::new();
    builder.add_from_string(result_str).unwrap();

    let item_name: Label = builder.object("item-name").unwrap();
    let item_icon: Image = builder.object("item-icon").unwrap();
    let item_desc: Label = builder.object("item-descr").unwrap();

    item_name.set_text(entry.name());

    let open_tag = format!(
      "<span foreground=\"{}\">",
      window
        .config
        .theme()
        .inner
        .matched_text_hl_colors
        .when_selected
    );
    let close_tag = "</span>";

    let mut name_c = entry.name().to_string();
    for (_, (index, chars)) in match_.0.iter().rev().enumerate() {
      name_c = name_c[0..*index]
        .to_string()
        .to_string()
        + &open_tag
        + &chars
        + close_tag
        + &name_c[index + chars.len()..];
    }
    
    item_name.set_markup(&name_c);

    item_icon.set_from_pixbuf(Some(&entry.icon()));

    item_icon.set_pixel_size(40);
    item_icon.set_margin(2);
    item_desc.set_text(entry.description());

    Self {
      builder,
      selected: false,
      match_,
      entry,
      index: 0,
      window,
    }
  }

  pub fn select(&mut self) {
    self.selected = true;
    let item_box: EventBox = self.builder.object("item-box").unwrap();
    item_box.style_context().add_class("selected");
  }

  pub fn deselect(&mut self) {
    self.selected = false;
    let item_box: EventBox = self.builder.object("item-box").unwrap();
    item_box.style_context().remove_class("selected");
  }

  pub fn setup(&self) {
    let item_box: EventBox = self.builder.object("item-box").unwrap();
    let result_notify = self.clone();
    item_box.connect_enter_notify_event(move |_, _| {
      let mut navigation = result_notify.window.navigation.lock().unwrap();
      navigation.select(result_notify.index);
      Inhibit(false)
    });

    let result_button = self.clone();
    item_box.connect_button_release_event(move |_, _| {
      let navigation = result_button.window.navigation.lock().unwrap();

      if let Some(selected) = navigation.selected {
        if result_button.window.config.main.daemon {
          result_button.window.window.hide();
          navigation.results[selected as usize]
            .entry
            .execute(result_button.window.clone());
        } else {
          navigation.results[selected as usize]
            .entry
            .execute(result_button.window.clone());
          std::process::exit(0);
        }
      }

      Inhibit(false)
    });
  }
}