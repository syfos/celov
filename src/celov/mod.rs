use crate::{fgit::{Git, watcher::WatchSignals}, sycode::core::Editor};
use std::sync::Arc;

/// A combined developer environment 
#[allow(dead_code)]
pub struct Celov {
  pub editor: Editor,
  pub fgit: Git,
  pub watcher_signal: Arc<WatchSignals>,
}

impl Celov {
  pub fn new() -> Result<Celov, Box<dyn std::error::Error>> {
    Ok(Celov {
      editor: Editor::new()?,
      fgit: Git::new("~/impl/rust/celov/")?,
      watcher_signal: WatchSignals::spawn()?,
    })
  }
}
