use std::result;

use crate::fgit::Git;
use git2::{Oid, Repository, Sort};

#[allow(dead_code)]
impl Git {
  /// Returns `list` of `local commits` present in repo.
  /// List order will always be sorted `topological` (`child commit` before `parent`) along with `time`(`newest` before `oldest`).
  /// Chance of faliure is rare.
  pub fn get_present_commits_list(repo: &Repository) -> result::Result<Vec<Oid>, git2::Error> {
    // Walk the commit graph.
    let mut revwalk = repo.revwalk()?;
    // use the head'oid as seed to the iterator.
    revwalk.push_head()?;
    // sort the oid into Topological and then sort with Time based ordering.
    // The parameter should always be -->
    // (Sort::TOPOLOGICAL | Sort::TIME)
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;

    // return
    revwalk.collect()
  }
}
