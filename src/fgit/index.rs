use std::result;

use git2::{Index, Repository, Status, StatusOptions, Statuses};

/// Each code has a meaning. Parse them.
///
/// Short code show status of file:
/// 1. In `Index` and in `Worktree` e.g `MM` for modified in both.
///
/// 2. In `Ours` vs `Their's` side during merge e.g `DD` for deleted by both.
#[allow(dead_code)]
pub struct StatusCode {
  pub short_status_code: String,
  pub file_path: String,
}

#[allow(dead_code)]
impl StatusCode {
  /// Returns the X's equivalent type from (XY) or simply the Index status of underlying file but in a char like 'A' for New item in Index.
  fn index_char(s: git2::Status) -> char {
    if s.contains(Status::INDEX_NEW) {
      'A'
    } else if s.contains(Status::INDEX_MODIFIED) {
      'M'
    } else if s.contains(Status::INDEX_DELETED) {
      'D'
    } else if s.contains(Status::INDEX_RENAMED) {
      'R'
    } else if s.contains(Status::INDEX_TYPECHANGE) {
      'T'
    } else {
      ' '
    }
  }

  /// Returns the Y's equivalent type (from XY) or simply the Worktree(unstaged) status of underlying file but in a char like 'M' for modified in worktree(unstaged).
  fn wt_char(s: git2::Status) -> char {
    if s.contains(Status::WT_MODIFIED) {
      'M'
    } else if s.contains(Status::WT_DELETED) {
      'D'
    } else if s.contains(Status::WT_TYPECHANGE) {
      'T'
    } else if s.contains(Status::WT_RENAMED) {
      'R'
    } else {
      ' '
    }
  }

  /// Conflict detection -- better to use as is.
  fn conflict_code(index: &git2::Index, path: &str) -> Option<&'static str> {
    for c in index.conflicts().ok()?.flatten() {
      if c
        .our
        .as_ref()
        .or(c.their.as_ref())
        .or(c.ancestor.as_ref())
        .map(|e| e.path == path.as_bytes())
        != Some(true)
      {
        continue;
      }
      return Some(
        match (c.ancestor.is_some(), c.our.is_some(), c.their.is_some()) {
          (true, false, false) => "DD",
          (false, true, false) => "AU",
          (true, true, false) => "UD",
          (false, false, true) => "UA",
          (true, false, true) => "DU",
          (false, true, true) => "AA",
          (true, true, true) => "UU",
          _ => "??",
        },
      );
    }
    None
  }

  /// Gives the statuses to iterate over
  /// Gives the statuses to iterate over
  fn get_statuses<'repo>(repo: &'repo Repository) -> std::result::Result<Statuses<'repo>, String> {
    let mut opts = StatusOptions::new();
    opts
      .include_untracked(true)
      .include_ignored(false)
      .recurse_untracked_dirs(true)
      .exclude_submodules(false)
      .renames_head_to_index(true)
      .renames_index_to_workdir(true)
      .no_refresh(false);

    match repo.statuses(Some(&mut opts)) {
      Ok(statuses) => Ok(statuses),
      Err(e) => Err(format!("Error while getting statuses: {e}")),
    }
  }

  /// Constructs the status code for file depending whether it is untracked(??), conflicted(DD, UA, AU) or normal everyday case(M ,MM, M)
  fn constructor(status: &Status, index: &Index, path: &str) -> String {
    // if status contains new file in worktree and the status of that file don't contains(intersects -> for more broader scope) Status::_bitflags_listed_ then it is a true so mark that as untracked (??).
    //
    // else if it is conflicted then get the possible status out of 7 possible variants of XY
    //
    // else just construct string depending on variations of X and Y
    if status.contains(git2::Status::WT_NEW)
      && !status.intersects(
        git2::Status::INDEX_NEW
          | git2::Status::INDEX_MODIFIED
          | git2::Status::INDEX_DELETED
          | git2::Status::INDEX_RENAMED
          | git2::Status::INDEX_TYPECHANGE,
      )
    {
      "??".to_string()
    } else if status.is_conflicted() {
      Self::conflict_code(index, path).unwrap_or("UU").to_string()
    } else {
      format!("{}{}", Self::index_char(*status), Self::wt_char(*status))
    }
  }
  /// Generates the `Vec<FileStatus>`.
  ///
  /// ```
  /// pub struct FileStatus {
  ///   pub code: String,
  ///   pub path: String,
  /// }
  /// ```
  ///
  /// Where Code is in `XY`, `X_` or `_Y` format depending on status of the file.  **{Note: _ means blank}**
  pub fn new(repo: &Repository) -> Vec<result::Result<StatusCode, String>> {
    let index = match repo.index() {
      Ok(idx) => idx,
      Err(e) => return vec![Err(e.to_string())],
    };
    let statuses = match Self::get_statuses(repo) {
      Ok(statuses) => statuses,
      Err(e) => return vec![Err(e)],
    };
    let mut vector = Vec::new();

    for entry in statuses.iter() {
      let path = entry.path().unwrap_or("<invalid utf-8>");
      let status = entry.status();
      vector.push(Ok(StatusCode {
        short_status_code: Self::constructor(&status, &index, path),
        file_path: path.to_string(),
      }));
    }
    vector
  }
}
