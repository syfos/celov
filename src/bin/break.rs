use std::{error, result};

use celov::sycode::{
  core::Editor,
  softwrap::new::{FittingSlices, Wrap},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn main() -> result::Result<(), Box<dyn error::Error>> {
  let mut editor = Editor::new()?;
  let mut fitting_slices = FittingSlices::default();

  let line = editor.rope.line(0).to_string();

  println!("bytes: {}", line.len());
  println!("graphemes: {}", line.graphemes(true).count());
  println!("width: {}", line.width());

  for (i, g) in line.graphemes(true).enumerate() {
    println!("{i}: width={} {g:?}", g.width());
  }

  for i in 0..editor.rope.len_lines() {
    Wrap::as_wrapped(
      &mut editor.softwrap,
      &editor.icu,
      &editor.rope,
      i,
      40usize,
      &mut fitting_slices,
    );
  }

  let slice = &editor.softwrap.displayed_lines.get(&0usize).unwrap();
  for s in *slice {
    println!("Width: {}", s.width());
    // println!("{:#?}", slice);
  }

  Ok(())
}
