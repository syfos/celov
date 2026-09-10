use crate::fgit::Git;
use git2::{Branch, Oid, Repository};

#[allow(dead_code)]
impl Git {
  /// Global helper function to tackle `String to Branch<'repo>` cases through Git
  /// Convert any valid `local` branch name into `Branch<'repo>`
  pub fn to_local_branch<'repo>(
    repo: &'repo Repository,
    attached: &str,
  ) -> anyhow::Result<Branch<'repo>, anyhow::Error> {
    Ok(repo.find_branch(attached, git2::BranchType::Local)?)
  }

  /// Get oid of any **branch: `Branch<'repo>`**
  pub fn get_oid(repo: &Repository, branch: &Branch) -> anyhow::Result<Oid, anyhow::Error> {
    return Ok(repo.find_commit(branch.get().target().unwrap())?.id());
  }

  /// The name `origin` is the default name given to the remote from which the repo has been cloned (could be **renamed**).
  ///
  /// An `upstream` is a branch on remote repo that the local repo's branch track.
  ///
  /// This function can return either of :
  ///
  /// `remote_name/branch_name` or simply `origin/bmame`
  /// `None` if there is no `Upstream`.
  /// `error: String` if there is an Error.
  ///
  pub fn get_upstream(branch: &Branch) -> anyhow::Result<Option<String>, anyhow::Error> {
    match branch.upstream() {
      Ok(b) => Ok(Some(b.name()?.unwrap().to_string())),
      Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
      Err(e) => Ok(Some(e.to_string())),
    }
  }
}
