use item::Item;
use ratatui::{
  prelude::{Buffer, Rect},
  style::{Style, Stylize},
  text::{Line, Span, Text},
  widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
  app_state::{ItemListState, ItemStore},
  colors::*,
};

fn item_line<'i>(item: &'i Item, state: &ItemListState) -> Line<'i> {
  let mut bold_style = Style::new().bold();
  let mut dim_style = Style::new().fg(DIM_TEXT_COLOR_RATATUI);

  match state.selected {
    Some(selected_item) if selected_item == item.id() => {
      bold_style = bold_style.underlined();
      dim_style = dim_style.underlined();
    }
    _ => (),
  }

  Line::from_iter(
    [
      Span::styled(" • ", dim_style),
      Span::styled(&item.meta().content, bold_style),
      Span::styled(" - ", dim_style),
    ]
    .into_iter()
    .chain(
      item
        .meta()
        .description
        .as_ref()
        .map(|d| Span::styled(d, dim_style)),
    ),
  )
}

pub struct ItemListWidget<'s> {
  pub item_store: &'s ItemStore,
  pub state:      &'s ItemListState,
}

impl Widget for ItemListWidget<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let Self { item_store, state } = self;

    let lines = item_store
      .item_iter()
      .into_iter()
      .map(|i| item_line(i, state));

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
