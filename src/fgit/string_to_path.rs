use crate::fgit::Git;
use anyhow::Context;
use std::path::{Path, PathBuf};
impl Git {
  /// Parses String into PathBuf via crate:  `Shellexpand`.
  /// Status : Accurate and Tested by `ipude`.
  pub fn string_to_path(path_string: &str) -> anyhow::Result<PathBuf> {
    let expanded = shellexpand::full(path_string)
      .with_context(|| format!("failed to expand path: `{path_string}`"))?;

    let canonical = Path::new(expanded.as_ref())
      .canonicalize()
      .with_context(|| format!("The expanded path do not exists or is inaccessible : `{expanded}` probably the `{path_string}` is wrong."))?;
    Ok(canonical)
  }
}
