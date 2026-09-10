use std::result;

use crate::fgit::{Git, branches::BranchesContainer};
use git2::{Branch, Repository};

/// Contains ahead-behind data.
#[allow(dead_code)]
#[derive(Clone)]
pub struct ABData {
  pub given_branch: String,
  pub other_branch: String,
  pub ahead: usize,
  pub behind: usize,
}

#[allow(dead_code)]
impl Git {
  /// 1. `Given branch` is matched against `every local branch` of repo, `excluding itself`.
  ///
  /// 2. Will return an empty [`Vec<ABData>`] if the repo has a single branch, or if `result` is [`Err`] i.e iterator over branches could not have been created.
  ///
  /// 3. May skip a branch from branches if its `name: String` can't be expanded into [`git2::Branch<'repo>`]
  ///
  /// 4. Will not `panic/propogate` error.
  pub fn get_ahead_behind(
    vector: &mut Vec<ABData>,
    repo: &Repository,
    current_branch: &Branch,
    branches: &[String],
  ) -> Vec<ABData> {
    for branch in branches {
      let other_branch = match Git::to_local_branch(repo, branch.as_str()) {
        Ok(v) => v,
        Err(_) => continue,
      };

      let current_branch_name = match current_branch.name() {
        Ok(Some(v)) => v.to_string(),
        Ok(None) => "<Invalid Utf-8>".to_string(),
        Err(_) => "<Invalid Branch/HEAD is detached>".to_string(),
      };

      let other_branch_name = match other_branch.name() {
        Ok(Some(v)) => v.to_string(),
        Ok(None) => "<Invalid Utf-8>".to_string(),
        Err(_) => "<Invalid Branch/HEAD is detached>".to_string(),
      };

      // skip same branch
      if current_branch_name != other_branch_name {
        let (ahead, behind) = match repo.graph_ahead_behind(
          current_branch.get().target().unwrap(),
          other_branch.get().target().unwrap(),
        ) {
          Ok(v) => v,
          Err(_) => continue,
        };

        vector.push(ABData {
          given_branch: current_branch_name,
          other_branch: other_branch_name,
          ahead,
          behind,
        });
      }
    }
    vector.to_vec()
  }

  pub fn safely_get_ahead_behind(
    repo: &Repository,
    current_branch: &Branch,
    result: &result::Result<BranchesContainer, String>,
  ) -> Vec<ABData> {
    let mut vector: Vec<ABData> = Vec::new();
    match &result {
      Ok(container) => {
        if container.is_single_branch() {
          vector
        } else {
          let branches = &container.vector_branches;
          Git::get_ahead_behind(&mut vector, repo, current_branch, branches)
        }
      }

      Err(_) => vector,
    }
  }
}
