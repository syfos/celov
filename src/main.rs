use crate::celov::Celov;

mod celov;
mod cmd;
mod fgit;
mod sycode;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  cmd::parser();
  let mut app = Celov::new()?;
  app.editor.run()?;
  Ok(())
}
