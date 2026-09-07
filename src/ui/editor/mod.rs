use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{fs, io::BufReader};

use ropey::Rope;

pub mod cursor;
pub mod edit;
pub mod helper;
pub mod new;
pub mod render;

#[derive(Default)]
pub struct Editor {
  pub scroll_offset: usize,
  pub rope: Rope,
  pub cursor: Cursor,
}

#[allow(dead_code)]
impl Editor {
  /// Parses String into PathBuf via crate: `Shellexpand`.
  pub fn string_to_path(path_string: &str) -> io::Result<PathBuf> {
    let expanded = shellexpand::full(path_string).map_err(|e| {
      Error::new(
        ErrorKind::InvalidInput,
        format!("failed to expand path `{path_string}`: {e}"),
      )
    })?;

    let canonical = Path::new(expanded.as_ref()).canonicalize().map_err(|e| {
      Error::new(
        e.kind(),
        format!("path `{expanded}` does not exist or is inaccessible (from `{path_string}`): {e}"),
      )
    })?;

    Ok(canonical)
  }

  pub fn new() -> io::Result<Editor> {
    let reader = BufReader::new(fs::File::open(Self::string_to_path(
      "~/impl/rust/fgit/src/bin/txt.txt",
    )?)?);
    Ok(Self {
      scroll_offset: 0,
      rope: Rope::from_reader(reader)?,
      cursor: Cursor::default(),
    })
  }
}

/// The cursor position in `(col, row)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);
