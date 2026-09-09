use crate::{git::Git, ui::editor::new::core::Editor, watcher::WatchSignals};
use std::sync::Arc;

#[allow(dead_code)]
pub struct App {
  pub editor: Editor,
  pub fgit: Git,
  pub watcher_signal: Arc<WatchSignals>,
}

impl App {
  pub fn new() -> Result<App, Box<dyn std::error::Error>> {
    Ok(App {
      editor: Editor::new()?,
      fgit: Git::new("~/impl/rust/fgit/")?,
      watcher_signal: WatchSignals::spawn()?,
    })
  }
}
