use std::{
  fs::{create_dir_all, read, write},
  path::PathBuf,
};

use dashmap::DashMap;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::launcher::util::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
  /// Used to represent a JSON String value
  String(String),
  /// Used to represent a JSON numerical value
  Number(u64),
  /// Used to represent a JSON boolean value
  Boolean(bool),
  /// Used to represent a JSON array of `Value`s
  Array(Vec<Value>),
  /// Used to represent a JSON object of `String`s to `Value`s
  Map(DashMap<String, Value>),
}

#[derive(Debug, Clone)]
pub struct ExtensionConfig {
  file: PathBuf,
  map: DashMap<String, Value>,
}

impl ExtensionConfig {
  /// Initialize a "new" extension config based on a extension's name.
  /// If the configuration file doesn't already exist it will be populated with an empty JSON object.
  /// Calling ExtensionConfig::new() should not be done in extensions as it is provided by the
  /// [ExtensionContext](ExtensionContext) struct.
  ///
  /// The extension's configuration file is stored in
  /// `(XDG_CONFIG_HOME or ~/.config)/dlauncher/extension_config`
  pub fn new(config: &Config, name: &str) -> Self {
    let extension_config_dir = config.dir().join("extension_config");
    create_dir_all(&extension_config_dir).unwrap();

    let file = extension_config_dir.join(format!("{}.json", name));
    let map = if file.exists() {
      debug!(
        "Loading extension config for {} from {}",
        name,
        file.display()
      );
      let contents = read(&file).unwrap();
      serde_json::from_slice(&contents).unwrap()
    } else {
      debug!(
        "Creating extension config for {} at {}",
        name,
        file.display()
      );
      let map = DashMap::new();
      write(&file, serde_json::to_vec(&map).unwrap()).unwrap();

      map
    };

    ExtensionConfig { map, file }
  }

  /// Set values. The key will always have to be a string, but the value can be any type that
  /// implements Into<Value>.
  /// Once the map in memory has been updated, it will save the current configuration to the disk
  ///
  /// # Example
  /// *When using config in an extension, use ExtensionContext.config to interface with
  /// the config instead of the way this example shows*
  /// ```rust
  /// use dlauncher::extension::config::ExtensionConfig;
  ///
  /// let config = ExtensionConfig::new(&config, "test");
  /// config.set("prefix", "time ");
  /// ```
  pub fn set<V: Into<Value>>(&self, key: &'static str, value: V) -> Option<Value> {
    let value = self.map.insert(key.to_string(), value.into());
    self.save();

    value
  }

  /// Get a value from the config. The key will always have to be a string, but the value can be any
  /// type that implements From<Value>.
  ///
  /// # Example
  /// *When using config in an extension, use ExtensionContext.config to interface with
  /// the config instead of the way this example shows*
  /// ```rust
  /// use dlauncher::extension::config::ExtensionConfig;
  ///
  /// let config = ExtensionConfig::new(&config, "test");
  ///
  /// let prefix: String = config.get("prefix").unwrap(); // panics when the key doesn't exist
  /// let some_other_key = config.get::<bool>("some_other_key");
  /// ```
  pub fn get<V: From<Value>>(&self, key: &'static str) -> Option<V> {
    self.map.get(key).map(|value| value.value().clone().into())
  }

  /// Check if a key exists in the extension config.
  pub fn contains_key(&self, key: &'static str) -> bool {
    self.map.contains_key(key)
  }

  /// Remove a key from the config. If the key doesn't exist this will return None, and not remove
  /// anything.
  pub fn remove<V: From<Value>>(&self, key: &'static str) -> Option<V> {
    let val = self.map.remove(key).map(|value| value.1.into());
    self.save();
    val
  }

  pub fn len(&self) -> usize {
    self.map.len()
  }

  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  /// Clear the entire configuration, and write it to the disk.
  pub fn clear(&self) {
    self.map.clear();
    self.save();
  }

  /// Save the current configuration to the disk.
  pub fn save(&self) {
    write(&self.file, serde_json::to_vec_pretty(&self.map).unwrap()).unwrap();
  }
}

impl From<&str> for Value {
  fn from(s: &str) -> Self {
    Value::String(s.to_string())
  }
}

impl From<String> for Value {
  fn from(s: String) -> Self {
    Value::String(s)
  }
}

impl From<u64> for Value {
  fn from(n: u64) -> Self {
    Value::Number(n)
  }
}

impl From<bool> for Value {
  fn from(b: bool) -> Self {
    Value::Boolean(b)
  }
}

impl From<Vec<Value>> for Value {
  fn from(v: Vec<Value>) -> Self {
    Value::Array(v)
  }
}

impl From<Vec<String>> for Value {
  fn from(v: Vec<String>) -> Self {
    Value::Array(v.into_iter().map(|s| s.into()).collect())
  }
}

impl From<Vec<u64>> for Value {
  fn from(v: Vec<u64>) -> Self {
    Value::Array(v.into_iter().map(|n| n.into()).collect())
  }
}

impl From<Vec<bool>> for Value {
  fn from(v: Vec<bool>) -> Self {
    Value::Array(v.into_iter().map(|b| b.into()).collect())
  }
}

impl From<DashMap<String, Value>> for Value {
  fn from(m: DashMap<String, Value>) -> Self {
    Value::Map(m)
  }
}

impl From<Value> for String {
  fn from(value: Value) -> Self {
    match value {
      Value::String(s) => s,
      _ => "".to_string(),
    }
  }
}

impl From<Value> for u64 {
  fn from(value: Value) -> Self {
    match value {
      Value::Number(n) => n,
      _ => 0,
    }
  }
}

impl From<Value> for bool {
  fn from(value: Value) -> Self {
    match value {
      Value::Boolean(b) => b,
      _ => false,
    }
  }
}

impl From<Value> for Vec<Value> {
  fn from(value: Value) -> Self {
    match value {
      Value::Array(a) => a,
      _ => vec![],
    }
  }
}

impl From<Value> for Vec<String> {
  fn from(value: Value) -> Self {
    match value {
      Value::Array(a) => a.into_iter().map(|v| v.into()).collect(),
      _ => vec![],
    }
  }
}

impl From<Value> for Vec<u64> {
  fn from(value: Value) -> Self {
    match value {
      Value::Array(a) => a.into_iter().map(|v| v.into()).collect(),
      _ => vec![],
    }
  }
}

impl From<Value> for Vec<bool> {
  fn from(value: Value) -> Self {
    match value {
      Value::Array(a) => a.into_iter().map(|v| v.into()).collect(),
      _ => vec![],
    }
  }
}

impl From<Value> for DashMap<String, Value> {
  fn from(value: Value) -> Self {
    match value {
      Value::Map(m) => m,
      _ => DashMap::new(),
    }
  }
}
