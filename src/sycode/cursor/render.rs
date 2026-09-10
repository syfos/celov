use crate::sycode::core::Cursor;
use crate::sycode::keymaps::editor::ModeType;
use ratatui::style::Color;
use ratatui::{Frame, style::Style};

#[allow(dead_code)]
impl Cursor {
  /// Note: This render function must be placed in the Tui::renderer's draw loop for instant update.
  /// Warn: Don't place it inside key input match block
  pub fn render(&mut self, mode: &ModeType, frame: &mut Frame) {
    match mode {
      ModeType::Normal | ModeType::Visual => {
        self.draw_block(frame);
      }
      ModeType::Insert => {
        self.draw_bar(frame);
      }
    }
  }

  fn draw_block(&mut self, frame: &mut Frame) {
    let buf = frame.buffer_mut();
    let cell = &mut buf[(self.0 as u16, self.1 as u16)];
    cell.set_style(Style::default().fg(Color::Black).bg(Color::White));
  }

  fn draw_bar(&mut self, frame: &mut Frame) {
    let buf = frame.buffer_mut();
    let cell = &mut buf[(self.0 as u16, self.1 as u16)];
    cell.set_symbol("|");
    cell.set_style(Style::default().fg(Color::White));
  }
}
