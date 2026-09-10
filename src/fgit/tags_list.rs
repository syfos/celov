use git2::Repository;

use crate::fgit::Git;

#[allow(dead_code)]
pub struct TagInfo {
  pub name: String,
  pub oid: git2::Oid,
  /// Whether notes/metadata is attached or not. If not then tag is lightweight instead of a git obj.
  pub annotated: bool,
}

#[allow(dead_code)]
impl Git {
  /// Get the entire tags of repo. TagInfo contains name of tag along with oid and annotated: bool (to tell whether it is lightweight/annotated)
  ///
  /// # Todo: Prepare data for anonated tags.
  /// ```
  ///pub struct TagInfo {
  ///  pub name: String,
  ///  pub oid: git2::Oid,
  ///  /// Whether notes/metadata is attached or not. If not then tag is lightweight instead of a git obj.
  ///  pub annotated: bool,
  ///}
  ///```
  pub fn get_tags_detailed(repo: &Repository) -> Result<Vec<TagInfo>, git2::Error> {
    let mut tags = Vec::new();

    repo.tag_foreach(|oid, name_bytes| {
      if let Ok(name) = std::str::from_utf8(name_bytes) {
        let short_name = name.trim_start_matches("refs/tags/");
        let annotated = repo.find_tag(oid).is_ok();
        tags.push(TagInfo {
          name: short_name.to_string(),
          oid,
          annotated,
        });
      }
      true
    })?;

    Ok(tags)
  }
}
