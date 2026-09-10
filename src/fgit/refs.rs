use git2::{Oid, Repository};

use crate::fgit::Git;

/// Container for derived loose refrences of repo.
/// Fields must be of type:`Vec<RefrenceData>`
/// ```
/// pub struct RefrenceData {
///   pub name: String,
///   pub oid: Oid,
/// }
/// ```
#[allow(dead_code)]
pub struct RefrenceContainer {
  pub heads: Vec<std::result::Result<RefrenceData, String>>,
}

impl RefrenceContainer {
  pub fn new(repo: &Repository) -> Self {
    Self {
      heads: Git::get_refs_from_glob(repo, "refs/heads/**"),
    }
  }
}

/// Contains underlying values of glob along with oid.
///
/// Obtained as vector of [`RefrenceData`] from [`Git::refs_from_glob`]
#[allow(dead_code)]
pub struct RefrenceData {
  pub matched_ref_name: String,
  pub matched_ref_oid: Oid,
}

#[allow(dead_code)]
#[allow(unused)]
impl Git {
  /// Returns a [`Vec<RefrenceData>`] containing the name and OID of each Git reference (`glob`).
  ///
  /// `glob` is a Git reference pattern, such as `refs/heads/**`.
  pub fn get_refs_from_glob(
    repo: &Repository,
    glob: &str,
  ) -> Vec<std::result::Result<RefrenceData, String>> {
    let mut vector = Vec::new();

    // return early
    let ref_iterator = match repo.references_glob(glob) {
      Ok(refr) => refr,
      Err(e) => {
        vector.push(Err(e.to_string()));
        return vector;
      }
    };

    // get Vec<RefrenceData>
    for entry in ref_iterator {
      // store error or move to next entry
      let matched_ref = match entry {
        Ok(matched_ref) => matched_ref,
        Err(e) => {
          //push
          vector.push(Err(e.to_string()));
          //skip
          continue;
        }
      };
      // pack
      vector.push(Ok(RefrenceData {
        matched_ref_name: matched_ref.name().unwrap_or("<Invalid Name>").to_string(),
        matched_ref_oid: matched_ref.target().unwrap(),
      }));
    }
    vector
  }
}
