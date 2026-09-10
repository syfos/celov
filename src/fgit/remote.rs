use crate::fgit::Git;
use git2::Repository;

/// Contains status of remote including its `name`, `url` and `pushurl`.
#[allow(dead_code)]
pub struct RemoteData {
  pub name: String,
  pub url: String,
  pub pushurl: String,
}

#[allow(dead_code)]
impl Git {
  ///
  /// By design this function returns either of :
  ///
  /// 1. [`Vec<RemoteData>`] if [`Repository::remotes()`] returns the listing of remotes.
  ///
  /// 2. [`String`] i.e Error is returned only if [`Repository::remotes()`] fails to list remotes.
  pub fn get_remotes(repo: &Repository) -> Result<Vec<RemoteData>, String> {
    let string_array = match repo.remotes() {
      Ok(arr) => arr,
      Err(e) => return Err(format!("Error while listing StringArray of remotes: {e}")),
    };

    let mut vector = Vec::new();

    for name_res in string_array.iter() {
      let name = match name_res {
        Ok(Some(n)) => n,
        Ok(None) => continue, // non-utf8-name, skip
        Err(_) => continue,   // unreadable-entry, skip
      };

      let remote = match repo.find_remote(name) {
        Ok(r) => r,
        Err(_) => continue, // failed to resolve remote -- skip
      };

      let remote_name = match remote.name() {
        Ok(Some(n)) => n.to_string(),
        _ => "<Invalid Remote Name>".to_string(),
      };

      let url = match remote.url() {
        Ok(u) => u.to_string(),
        Err(_) => "<Invalid Remote Url>".to_string(),
      };

      let pushurl = match remote.pushurl() {
        Ok(Some(p)) => p.to_string(),
        _ => "<None>".to_string(),
      };

      vector.push(RemoteData {
        name: remote_name,
        url,
        pushurl,
      });
    }

    Ok(vector)
  }
}
