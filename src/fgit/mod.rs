use std::{result};

use crate::fgit::{
  ahead_behind::ABData, branches::BranchesContainer, config::ConfigData, head::HeadCondition,
  index::StatusCode, refs::RefrenceContainer, remote::RemoteData, repo_state::RepoState,
  stash_list::StashData, tags_list::TagInfo,
};
use git2::{Oid, Repository};

pub mod ahead_behind;
pub mod branches;
pub mod commit_log;
pub mod config;
pub mod head;
pub mod index;
pub mod refs;
pub mod remote;
pub mod repo_state;
pub mod stash_list;
pub mod string_to_path;
pub mod tags_list;
pub mod utils;
pub mod watcher;

/// Fgit's data struct for Git.
#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub head: HeadCondition,
  pub state: RepoState,
  pub refs: RefrenceContainer,
  pub remotes: result::Result<Vec<RemoteData>, String>,
  pub config: Vec<result::Result<ConfigData, String>>,
  pub git_status: Vec<result::Result<StatusCode, String>>,
  pub branches_container: result::Result<BranchesContainer, String>,
  pub ahead_behind: Vec<ABData>,
  pub commits: Vec<Oid>,
  pub stash_list: Vec<StashData>,
  pub tag_list: Vec<TagInfo>,
}

#[allow(dead_code)]
impl Git {
  /// Compiles everything into a single structure.
  /// Can be used for Repowide refresh if called again.
  pub fn new(path: &str) -> anyhow::Result<Self> {
    let mut repo = Repository::open(Git::string_to_path(path)?)?;
    let head = HeadCondition::new(&repo)?;
    let refs = RefrenceContainer::new(&repo);

    let config = Git::get_config(&repo);
    let git_status = StatusCode::new(&repo);
    let remotes = Git::get_remotes(&repo);
    let branches_container = Git::get_branches(&repo);
    let ahead_behind = Git::safely_get_ahead_behind(
      &repo,
      &head.get_attached(&repo)?.unwrap(),
      &branches_container,
    );
    let commits = Git::get_present_commits_list(&repo)?;
    let stash_list = Git::get_stash_list(&mut repo)?;
    let tag_list = Git::get_tags_detailed(&repo)?;
    let state = Git::get_repo_state(&repo);

    Ok(Self {
      repo,
      head,
      refs,
      config,
      git_status,
      remotes,
      branches_container,
      commits,
      stash_list,
      tag_list,
      state,
      ahead_behind,
    })
  }
}
