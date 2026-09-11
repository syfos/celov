use ropey::Rope;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{fs, io::BufReader};

use crate::sycode::cursor::cursor_struct::{Cursor, CursorNew};
use crate::sycode::keymaps::modal::ModeType;
use crate::sycode::splits::Splits;

#[derive(Default)]
pub struct Editor {
  pub scroll_offset: usize,
  pub rope: Rope,
  pub cursor: CursorNew,
  pub mode: ModeType,
  pub splits: Splits,
  pub viewport_height: usize,
  pub viewport_width: usize,
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
      "~/impl/rust/celov/src/bin/txt.txt",
    )?)?);
    Ok(Self {
      scroll_offset: 0,
      rope: Rope::from_reader(reader)?,
      cursor: CursorNew::default(),
      mode: ModeType::Normal,
      splits: Splits::default(),
      viewport_height: 0,
      viewport_width: 0,
    })
  }
}

