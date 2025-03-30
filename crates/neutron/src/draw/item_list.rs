use item::Item;
use ratatui::{
  prelude::{Buffer, Rect},
  style::{Style, Stylize},
  text::{Line, Span, Text},
  widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{app_state::ItemStore, colors::*};

fn item_line(item: &Item) -> Line {
  let bold_style = Style::new().bold();
  let dim_style = Style::new().fg(DIM_TEXT_COLOR_RATATUI);

  Line::from_iter(
    [
      Span::styled(" • ", dim_style),
      Span::styled(&item.data().content, bold_style),
      Span::styled(" - ", dim_style),
    ]
    .into_iter()
    .chain(
      item
        .data()
        .description
        .as_ref()
        .map(|d| Span::styled(d, dim_style)),
    ),
  )
}

pub struct ItemListWidget<'s> {
  pub item_store: &'s ItemStore,
}

impl Widget for ItemListWidget<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let lines = self.item_store.item_iter().into_iter().map(item_line);

    let block = Block::bordered()
      .border_type(BorderType::Rounded)
      .border_style(Style::new().fg(NORMAL_BORDER_COLOR_RATATUI))
      .title(Line::styled(
        "Items",
        Style::new().fg(NORMAL_TEXT_COLOR_RATATUI).bold(),
      ));
    let par = Paragraph::new(Text::from_iter(lines)).block(block);

    par.render(area, buf);
  }
}
