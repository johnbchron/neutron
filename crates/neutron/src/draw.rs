use std::sync::Arc;

use ratatui::{
  Frame,
  style::Stylize,
  text::{Line, Text},
  widgets::{Block, Paragraph},
};
use tokio::{sync::Mutex, time::interval};

use crate::{
  FRAME_DURATION,
  app_state::AppState,
  colors::{BACKGROUND_COLOR_RATATUI, NORMAL_TEXT_COLOR_RATATUI},
};

pub async fn draw_task(state: Arc<Mutex<AppState>>) {
  let mut terminal = ratatui::init();
  let mut interval = interval(FRAME_DURATION);

  loop {
    interval.tick().await;

    let state_snapshot = {
      let state = state.lock().await;
      AppState::clone(&state)
    };

    terminal
      .draw(draw(state_snapshot))
      .expect("failed to draw frame");
  }
}

fn draw(state: AppState) -> impl FnOnce(&mut Frame) {
  move |frame: &mut Frame| {
    let block = Block::new()
      .fg(NORMAL_TEXT_COLOR_RATATUI)
      .bg(BACKGROUND_COLOR_RATATUI);
    let par = Paragraph::new(Text::from(vec![
      Line::from("Hello World!"),
      Line::from(format!("Counter: {}", state.counter)),
    ]))
    .block(block);

    frame.render_widget(par, frame.area());
  }
}
