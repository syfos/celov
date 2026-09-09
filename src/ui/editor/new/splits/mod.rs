use ratatui::{
  Frame,
  layout::{Constraint, Direction, Rect},
  widgets::{Block, Borders, Paragraph},
};

/// Contains splits based on two directions `Horizontal` and `Vertical`.
#[derive(Default)]
pub struct Splits {
  pub horizontal: SplitAxis,
  pub vertical: SplitAxis,
}

/// Pick `Bottom` for rendering `Vertical` splits and `Right` for rendering `Horizontal` splits.
pub enum SplitSeperator {
  Bottom,
  Right,
}

impl SplitSeperator {
  /// Returns the equivalent [`Borders`] bitflag.
  /// ```
  /// match self {
  ///   SplitSeperator::Bottom => Borders::BOTTOM,
  ///   SplitSeperator::Right => Borders::RIGHT,
  /// }
  /// ```
  fn as_borders(&self) -> Borders {
    match self {
      SplitSeperator::Bottom => Borders::BOTTOM,
      SplitSeperator::Right => Borders::RIGHT,
    }
  }
}

/// Contains count and splits for two underlying directions `Horizontal` and `Vertical`.
#[derive(Default)]
pub struct SplitAxis {
  pub count: u32,
  pub splits: Vec<Rect>,
}

#[allow(dead_code)]
impl SplitAxis {
  /// Increments splits counter correctly to prevent `count == 1` case as `screen_area/1` results in no splits.
  ///
  /// On first `<C-w>v` or `<C-w>h` sequence the counter will be incremented by `2` to equally divide `screen_area` into `two splits`.
  ///
  /// On second sequence this will increment `only by 1`.
  pub fn increment_count(&mut self) {
    if self.splits.is_empty() {
      self.count = self.count.saturating_add(2);
    } else {
      self.count = self.count.saturating_add(1);
    }
  }

  pub fn decrement_count(&mut self) {
    if self.splits.len() == 2 {
      self.count = self.count.saturating_sub(2);
    } else {
      self.count = self.count.saturating_sub(1);
    }
  }

  /// Splits current buffer into equal splits according to the given [`Direction`] i.e only `Vertical` and `Horizontal`.
  pub fn split(&mut self, frame_area: Rect, direction: Direction) {
    let constraints = vec![Constraint::Ratio(1, self.count); self.count as usize];
    self.splits = ratatui::layout::Layout::default()
      .direction(direction)
      .constraints(constraints)
      .split(frame_area)
      .to_vec()
  }

  pub fn del_split(&mut self) {
    self.splits.pop();
  }

  /// Renders splits according to the given [`SplitSeperator`].
  /// Enter `SplitSeperator::Bottom` when rendering `Vertical` splits and use `SplitSeperator::Right` when rendering `Horizontal` splits.
  pub fn render_widget(&self, frame: &mut Frame, seperator: SplitSeperator) {
    let seperator = seperator.as_borders();
    for (i, chunk) in self.splits.iter().enumerate() {
      let borders = if (i + 1) < self.splits.len() {
        seperator
      } else {
        Borders::NONE
      };
      let block = Block::default().borders(borders);
      let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);
      frame.render_widget(paragraph, *chunk);
    }
  }
}

impl Splits {
  fn calculate(&mut self, frame_area: Rect) {
    self
      .vertical
      .split(frame_area, ratatui::layout::Direction::Vertical);

    self
      .horizontal
      .split(frame_area, ratatui::layout::Direction::Horizontal);
  }

  pub fn render(&mut self, frame: &mut Frame) {
    self.calculate(frame.area());
    self.vertical.render_widget(frame, SplitSeperator::Bottom);
    self.horizontal.render_widget(frame, SplitSeperator::Right);
  }
}
