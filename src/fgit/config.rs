use std::result;

use crate::fgit::Git;
use git2::Repository;

/// Individual entry and value of the `.git/config`.
#[allow(dead_code)]
pub struct ConfigData {
  pub entry: String,
  pub value: String,
}

impl Git {
  /// Get the vector of [`Result<ConfigData, String>`].
  /// May contain only one index if :
  /// 1. Failed reading repo's [`Config`].
  /// 2. Failed creating snapshot of [`Config`].
  /// 3. Recieved error instead of [`ConfigEntries<'_>`]
  ///
  /// `Note`:  String(Error) could be stored in any index.
  pub fn get_config(repo: &Repository) -> Vec<result::Result<ConfigData, String>> {
    // return early
    let mut config = match repo.config() {
      Ok(config) => config,
      Err(e) => return vec![Err(format!("Error while reading config: {e}"))],
    };

    // Cache config for further iteration.
    let snapshot = match config.snapshot() {
      Ok(s) => s,
      Err(e) => return vec![Err(format!("Error while snapshoting: {e}"))],
    };

    // get iterator over all types of config entries.
    let mut entries = match snapshot.entries(None) {
      Ok(e) => e,
      Err(e) => {
        return vec![Err(format!(
          "Error while getting config entries of config's snapshot: {e}"
        ))];
      }
    };
    let mut vector = Vec::new();

    // Iterate -> match Some(entry)
    // Stop -> if None returned (often at last)
    while let Some(entry) = entries.next() {
      let entry = match entry {
        Ok(entry) => entry,
        // Don't return just push and skip.
        Err(e) => {
          vector.push(Err(e.to_string()));
          continue;
        }
      };

      vector.push(Ok(ConfigData {
        // Never use --> ? (try operator)
        entry: entry.name().unwrap_or("<Invalid Entry Name>").to_string(),
        value: entry.value().unwrap_or("<Invalid Entry Value>").to_string(),
      }));
    }
    vector
  }
}
