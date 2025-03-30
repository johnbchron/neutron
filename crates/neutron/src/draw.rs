mod hello_world_screen;
mod item_list;

use std::{ops::Deref, sync::Arc};

use ratatui::{prelude::*, widgets::Block};
use tokio::{sync::Mutex, time::interval};

use self::item_list::ItemListWidget;
use crate::{FRAME_DURATION, app_state::AppState, colors::*};

pub async fn draw_task(state: Arc<Mutex<AppState>>) {
  let mut terminal = ratatui::init();
  let mut interval = interval(FRAME_DURATION);

  loop {
    interval.tick().await;

    let state = state.lock().await;
    terminal.draw(draw(state)).expect("failed to draw frame");
  }
}

fn draw(state: impl Deref<Target = AppState>) -> impl FnOnce(&mut Frame) {
  move |frame: &mut Frame| {
    let bg_block = Block::new().style(
      Style::new()
        .fg(NORMAL_TEXT_COLOR_RATATUI)
        .bg(BACKGROUND_COLOR_RATATUI),
    );
    frame.render_widget(bg_block, frame.area());

    let item_list = ItemListWidget {
      item_store: &state.items,
    };

    frame.render_widget(item_list, frame.area());
  }
}
