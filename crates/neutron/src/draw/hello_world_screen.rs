use ratatui::{
  layout::{Constraint, Layout},
  prelude::{Buffer, Rect},
  style::{Style, Stylize},
  text::{Line, Text},
  widgets::{
    Block, BorderType, Borders, Padding, Paragraph, Widget, WidgetRef,
  },
};

use crate::colors::{
  ACTIVE_BORDER_COLOR_RATATUI, BACKGROUND_COLOR_RATATUI,
  NORMAL_TEXT_COLOR_RATATUI,
};

pub struct HelloWorldScreen;

impl Widget for HelloWorldScreen {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let block = Block::new()
      .fg(NORMAL_TEXT_COLOR_RATATUI)
      .bg(BACKGROUND_COLOR_RATATUI);
    block.render_ref(area, buf);

    let block = block
      .borders(Borders::all())
      .border_style(Style::new().fg(ACTIVE_BORDER_COLOR_RATATUI).bold())
      .border_type(BorderType::Rounded)
      .padding(Padding::horizontal(1));
    let par = Paragraph::new(Text::from(vec![Line::from("Hello World!")]))
      .block(block)
      .centered();
    let par_width = par.line_width() as u16;

    let vertical_layout = Layout::vertical([
      Constraint::Fill(1),
      Constraint::Length(par.line_count(par_width) as u16),
      Constraint::Fill(1),
    ])
    .split(area);
    let horizontal_layout = Layout::horizontal([
      Constraint::Fill(1),
      Constraint::Length(par_width),
      Constraint::Fill(1),
    ])
    .split(vertical_layout[1]);

    par.render(horizontal_layout[1], buf);
  }
}
