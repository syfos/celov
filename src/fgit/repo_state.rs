use git2::{Repository, RepositoryState};

use crate::fgit::Git;
/// Exact variants for the enum variants of RepositoryState.
pub enum RepoState {
  Clean,
  Merging,
  Rebasing,
  RebaseMerge,
  SingleCherryPick,
  MultiCherryPick,
  SingleRevert,
  MultiRevert,
  Bisect,
  ApplyingMailBox,
  MailBoxOrRebase,
}

#[allow(dead_code)]
impl Git {
  /// Returns only **one** active state of Repository.
  pub fn get_repo_state(repo: &Repository) -> RepoState {
    let state = repo.state();
    match &state {
      RepositoryState::Clean => RepoState::Clean,
      RepositoryState::Merge => RepoState::Merging,
      RepositoryState::Revert => RepoState::SingleRevert,
      RepositoryState::RevertSequence => RepoState::MultiRevert,
      RepositoryState::CherryPick => RepoState::SingleCherryPick,
      RepositoryState::CherryPickSequence => RepoState::MultiCherryPick,
      RepositoryState::RebaseMerge => RepoState::RebaseMerge,
      RepositoryState::Rebase | RepositoryState::RebaseInteractive => RepoState::Rebasing,
      RepositoryState::Bisect => RepoState::Bisect,
      RepositoryState::ApplyMailbox => RepoState::ApplyingMailBox,
      RepositoryState::ApplyMailboxOrRebase => RepoState::MailBoxOrRebase,
    }
  }
}
