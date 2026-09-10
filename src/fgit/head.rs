use git2::{Branch, Oid};
use git2::{ErrorCode, Repository};

use crate::fgit::Git;

/// This enum contains status of Head.
/// `Attached(String)` if head is a branch.
/// `Detached(Oid)` if head is'nt a branch but points to a commit.
/// `Error(String)` Serious errors worth displaying in tui.
/// `Unborn` if head is unborn.
///
/// If `head.is_attached()` returns true then look for `head.get_attached(repo: &Repository)` to get **Current Branch**.
#[allow(dead_code)]
pub enum HeadCondition {
  Attached(String),
  Detached(Oid),
  Error(String),
  Unborned,
}

// Helper functions for Head
#[allow(dead_code)]
impl HeadCondition {
  /// Returns true if `Head::Attached(_)` matches.
  pub fn is_attached(&self) -> bool {
    matches!(self, HeadCondition::Attached(_))
  }
  /// Returns true if `Head::Detached(_)` matches.
  pub fn is_detached(&self) -> bool {
    matches!(self, HeadCondition::Detached(_))
  }
  /// Returns true if `Head::Error(_)` matches.
  pub fn is_error(&self) -> bool {
    matches!(self, HeadCondition::Error(_))
  }
  /// Returns true if `Head::Unborn` matches.
  pub fn is_unborn(&self) -> bool {
    matches!(self, HeadCondition::Unborned)
  }

  /// Returns the current `Branch<'repo>` you are on if `Head` is attached. No need to call :
  /// ```
  /// if head.is_attached() {
  ///   head.get_attached()?.unwrap()
  /// }
  /// ```
  pub fn get_attached<'repo>(
    &self,
    repo: &'repo Repository,
  ) -> anyhow::Result<Option<Branch<'repo>>, anyhow::Error> {
    match self {
      HeadCondition::Attached(name) => {
        let branch = Git::to_local_branch(repo, &name.to_string())?;
        Ok(Some(branch))
      }
      _ => Ok(None),
    }
  }

  /// Returns oid if Head is deatched but points to a commit.
  pub fn get_detached(&self) -> Option<Oid> {
    match self {
      HeadCondition::Detached(oid) => Some(*oid),
      _ => None,
    }
  }

  /// Returns important error for displaying.
  pub fn get_error(&self) -> Option<String> {
    match self {
      HeadCondition::Error(err) => Some(err.to_string()),
      _ => None,
    }
  }
}

#[allow(dead_code)]
impl HeadCondition {
  /// Resolves status of Head. See enum `Head` for more details.
  pub fn new(repo: &Repository) -> anyhow::Result<HeadCondition, anyhow::Error> {
    match repo.head() {
      Ok(head) => {
        if head.is_branch() {
          // Make sure it must be repo.shorthand()? not repo.name()?
          Ok(HeadCondition::Attached(head.shorthand()?.to_string()))
        } else {
          match head.target() {
            Some(oid) => Ok(HeadCondition::Detached(oid)),
            None => Ok(HeadCondition::Error(
              "Detached HEAD but points to no Commit.".to_string(),
            )),
          }
        }
      }
      Err(e) if e.code() == ErrorCode::UnbornBranch => Ok(HeadCondition::Unborned),

      Err(e) => Ok(HeadCondition::Error(e.to_string())),
    }
  }
}
