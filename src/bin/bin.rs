use anyhow::Context;
use ropey::Rope;
use std::path::{Path, PathBuf};
use std::{fs, io::BufReader};

fn string_to_path(path_string: &str) -> anyhow::Result<PathBuf> {
  let expanded = shellexpand::full(path_string)
    .with_context(|| format!("failed to expand path: `{path_string}`"))?;

  let canonical = Path::new(expanded.as_ref())
      .canonicalize()
      .with_context(|| format!("The expanded path do not exists or is inaccessible : `{expanded}` probably the `{path_string}` is wrong."))?;
  Ok(canonical)
}

#[allow(unused)]
fn main() -> anyhow::Result<()> {
  let mut rope = Rope::from_reader(BufReader::new(fs::File::open(string_to_path(
    "~/impl/rust/fgit/src/bin/txt.txt",
  )?)?))?;
  println!("chars: {}", rope.len_chars());
  println!("lines: {}", rope.len_lines());

  for i in 0..rope.len_lines() {
    println!("line {i}: {:?}", rope.line(i));
  }
  Ok(())
}
