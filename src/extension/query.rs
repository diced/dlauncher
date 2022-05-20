#[derive(Debug, Clone)]
pub struct Query(String, String);

impl Query {
  /// Parses a string into Query
  pub fn from_str(query: &str) -> Query {
    let query_parts = query.split_once(' ').unwrap_or((query, ""));
    Query(query_parts.0.to_string(), query_parts.1.to_string())
  }

  /// Get the prefix of a query, for example
  /// "something test", the prefix is "something"
  pub fn prefix(&self) -> &str {
    &self.0
  }

  /// Get the rest of the query (arguments, etc.)
  ///
  /// For example, "something test", the query is "test"
  pub fn query(&self) -> &str {
    &self.1
  }

  /// If you do not want to listen to a prefix then you can just read the entire query
  pub fn all(&self) -> String {
    format!("{} {}", self.0, self.1)
  }
}
