use crossterm::event::KeyCode;
#[allow(dead_code)]
/// Event manager for `Fgit`.
pub struct EventManager {
  pub mode: ModeType,
  pub signal: IoSignal,
  pub pending_prefix: KeyCode,
}

impl EventManager {
  pub fn new() -> Self {
    Self {
      mode: ModeType::Normal,
      signal: IoSignal::None,
      pending_prefix: KeyCode::Null,
    }
  }
}

impl Default for EventManager {
  fn default() -> Self {
    Self::new()
  }
}

#[allow(dead_code)]
pub enum IoSignal {
  Vsplit,
  Hsplit,
  DelVsplit,
  DelHsplit,
  Left,
  Right,
  Up,
  Down,
  Quit,
  None,
}

#[allow(dead_code)]
#[derive(Default, PartialEq)]
pub enum ModeType {
  #[default]
  Normal,
  Visual,
  Insert,
}
