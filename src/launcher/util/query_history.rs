use crate::launcher::util::config::Config;
use dashmap::DashMap;
use log::debug;
use std::{
  fs::{read, write},
  path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct QueryHistory {
  file: PathBuf,
  map: DashMap<String, String>,
}

impl QueryHistory {
  pub fn new(config: Config) -> Self {
    let file = config.dir().join("query_history.json");
    let map = if file.exists() {
      debug!(
        "Loading query_history located at: {}",
        file.display()
      );
      let contents = read(&file).unwrap();
      serde_json::from_slice(&contents).unwrap()
    } else {
      debug!(
        "Creating an empty query_history located at: {}",
        file.display()
      );
      let map = DashMap::new();
      write(&file, serde_json::to_vec(&map).unwrap()).unwrap();

      map
    };

    QueryHistory { map, file }
  }

  pub fn find(&self, query: impl Into<String>) -> Option<String> {
    self.map.get(&query.into()).map(|t| t.value().clone())
  }

  pub fn save_query(&self, query: impl Into<String>, item_name: impl Into<String>) -> Option<String> {
    let query = query.into();
    let item_name = item_name.into();

    let value = self.map.insert(query.clone(), item_name);
    self.save();

    value
  }

  pub fn save(&self) {
    write(&self.file, serde_json::to_vec_pretty(&self.map).unwrap()).unwrap();
  }
}