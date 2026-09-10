mod app;
mod cmd;
mod git;
mod sycode;
mod watcher;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  cmd::parser();
  let mut app = crate::app::App::new()?;
  app.editor.run()?;
  Ok(())
}
