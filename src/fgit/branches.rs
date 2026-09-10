use std::result::Result;

use crate::fgit::Git;
use git2::Repository;

#[allow(dead_code)]
pub struct BranchesContainer {
  pub vector_branches: Vec<String>,
  pub error: BranchesError,
}

#[allow(dead_code)]
pub struct BranchesError {
  pub serious: Vec<String>,
  pub vector_non_utf8_branches: Vec<String>,
}

#[allow(dead_code)]
impl BranchesContainer {
  /// Returns `true` if `error.serious` contain at lest one error.
  pub fn has_serious_err(&self) -> bool {
    !self.error.serious.is_empty()
  }
  /// Returns `true` if `error.vector_non_utf8_branches` contain at lest one error.
  pub fn has_non_utf8_branches(&self) -> bool {
    !self.error.vector_non_utf8_branches.is_empty()
  }

  /// Returns `true` if `vector_branches` contain at lest one branch.
  pub fn is_branches_nil(&self) -> bool {
    self.vector_branches.is_empty()
  }

  /// Returns `true` if `vector_branches` has exactly one item.
  pub fn is_single_branch(&self) -> bool {
    self.vector_branches.len() == 1
  }
}

#[allow(dead_code)]
impl Git {
  /// Returns [`BranchesContainer`] if iterator of branches [`git2::Branches<'repo>`] can be created.
  ///
  /// # BranchesContainer
  ///
  /// ```
  ///pub struct BranchesContainer {
  ///  // this contains valid branches
  ///  pub vector_branches: Vec<String>,
  ///  // this contains malformed branches.
  ///  pub errors: BranchesError,
  ///}
  ///
  ///pub struct BranchesError {
  ///   pub serious: Vec<String>,
  ///   pub vector_non_utf8_branches: Vec<String>,
  ///}
  /// ```
  ///
  /// # Must use :
  /// ```
  ///impl BranchesContainer {
  ///  /// Returns `true` if `error.serious` contain at lest one error.
  ///  pub fn has_serious_err(&self) -> bool {
  ///    !self.error.serious.is_empty()
  ///  }
  ///
  ///  /// Returns `true` if `error.vector_non_utf8_branches` contain at lest one error.
  ///  pub fn has_non_utf8_branches(&self) -> bool {
  ///    !self.error.serious.is_empty()
  ///  }
  ///}
  /// ```
  ///
  pub fn get_branches(repo: &Repository) -> Result<BranchesContainer, String> {
    let mut vector_branches: Vec<String> = Vec::new();
    let mut non_utf_8: Vec<String> = Vec::new();
    let mut serious: Vec<String> = Vec::new();

    // Map error and return early
    let branches = repo
      .branches(Some(git2::BranchType::Local))
      .map_err(|e| e.to_string())?;

    for branch in branches {
      let (branch, _btype) = match branch {
        Ok(v) => v,

        // Serious error related to malfunctioning of repository.
        // related to libgit2's core api
        Err(e) => {
          serious.push(e.to_string());
          continue;
        }
      };
      match branch.name() {
        // Pick only valid names in vector_branches.
        Ok(Some(b)) => vector_branches.push(b.to_string()),

        // Non utf-8 names must go to vector_errors.
        Ok(None) => {
          let lossy_name = String::from_utf8_lossy(branch.get().name_bytes());
          let oid = branch
            .get()
            .target()
            .map(|o| o.to_string()) // if is Some(o: Oid) then convert to Some(o: String)
            .unwrap_or_else(|| "<unresolved_oid>".to_string()); // unwrap if Some(o) or else put "<unresolved_oid>"

          // Don't mix with Ok(name) as it must only contain valid branch names.
          // Send as Err(msg) to avoid sudden runtime crash.
          non_utf_8.push(format!("{lossy_name}: points to {oid}"))
        }
        Err(e) => serious.push(e.to_string()),
      }
    }

    Ok(BranchesContainer {
      vector_branches,
      error: BranchesError {
        serious,
        vector_non_utf8_branches: non_utf_8,
      },
    })
  }
}
