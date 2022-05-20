use std::sync::Arc;

use crate::launcher::result::ResultWidget;

use super::util::query_history::QueryHistory;

#[derive(Debug, Clone)]
pub struct Navigation {
  pub results: Vec<ResultWidget>,
  pub query_history: Arc<QueryHistory>,
  pub selected: Option<u16>,
}

impl Navigation {
  pub fn new(query_history: Arc<QueryHistory>) -> Self {
    Self {
      results: vec![],
      selected: None,
      query_history,
    }
  }

  pub fn select_default(&mut self, query: &str) {
    let previous = self.query_history.find(query);

    if let Some(previous) = previous {
      for (i, result) in self.results.iter().enumerate() {
        if result.entry.name() == previous {
          self.select(i as u16);
          break;
        }
      }
    } else {
      self.select(0);
    }
  }

  pub fn set_indicies(&mut self) {
    for (i, result) in self.results.iter_mut().enumerate() {
      result.index = i as u16;
    }
  }

  pub fn select(&mut self, mut index: u16) {
    if index as usize >= self.results.len() {
      index = 0;
    }

    if let Some(selected) = self.selected {
      if selected >= self.results.len() as u16 {
        self.selected = None;
      } else {
        self.results[selected as usize].deselect();
      }
    }

    if !self.results.is_empty() {
      self.selected = Some(index);
      self.results[index as usize].select();
    } else {
      self.selected = None;
    }
  }

  pub fn go_up(&mut self) {
    if let Some(selected) = self.selected {
      if selected > 0 {
        self.select(selected - 1);
      } else {
        self.select(self.results.len() as u16 - 1);
      }
    }
  }

  pub fn go_down(&mut self) {
    if let Some(selected) = self.selected {
      let next = selected + 1;
      if next < self.results.len() as u16 {
        self.select(next);
      } else {
        self.select(0);
      }
    }
  }
}
