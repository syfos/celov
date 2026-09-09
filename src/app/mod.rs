use crate::{ui::Tui, watcher::WatchSignals};
use std::sync::Arc;

pub struct App {
  pub tui: Tui,
  pub watcher_signal: Arc<WatchSignals>,
}

impl App {
  pub fn new() -> Result<App, Box<dyn std::error::Error>> {
    Ok(App {
      tui: Tui::new()?,
      watcher_signal: WatchSignals::spawn()?,
    })
  }
}
