use crate::fgit::Git;
use git2::{Oid, Repository};
use std::result;

#[allow(dead_code)]
pub struct StashData {
  pub index: usize,
  pub message: String,
  pub oid: Oid,
}

#[allow(dead_code)]
impl Git {
  /// Returns [`StashData`]. Chances of error: `rare`.
  /// ```
  /// pub struct StashData {
  ///   pub index: usize,
  ///   pub message: String,
  ///   pub oid: Oid,
  /// }
  /// ```
  pub fn get_stash_list(repo: &mut Repository) -> result::Result<Vec<StashData>, git2::Error> {
    let mut stashes = Vec::new();

    repo.stash_foreach(|index, message, oid| {
      stashes.push(StashData {
        index,
        message: message.to_string(),
        oid: *oid,
      });
      true // return true to keep iterating, false to stop early
    })?;

    Ok(stashes)
  }
}
