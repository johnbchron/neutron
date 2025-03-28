mod app_state;
mod colors;
mod commands;
mod draw;
mod events;

use std::{sync::Arc, time::Duration};

use tokio::{
  sync::{
    Mutex,
    mpsc::{self},
  },
  time::interval,
};

use self::{
  app_state::AppState, commands::command_runner, draw::draw_task,
  events::event_handler,
};

const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 60);

struct ItemId(ulid::Ulid);

struct Item {
  id:   ItemId,
  deps: DependencySet,
}

enum DependencySet {
  All(Vec<ItemId>),
}

#[tokio::main]
async fn main() -> Result<(), ()> {
  let state = Arc::new(Mutex::new(AppState::default()));

  let (commands, command_rx) = mpsc::channel(100);

  tokio::select! {
    res = tokio::spawn(draw_task(state.clone())) => {
      ratatui::restore();
      println!("draw_task task exited: {res:?}");
    },
    res = tokio::spawn(command_runner(state.clone(), command_rx)) => {
      ratatui::restore();
      println!("command_runner task exited: {res:?}");
    },
    res = tokio::spawn(event_handler(commands.clone())) => {
      ratatui::restore();
      println!("event_handler task exited: {res:?}");
    },
    _ = tokio::spawn(shutdown_task(state.clone())) => {
      ratatui::restore();
    },
  };

  ratatui::restore();

  Ok(())
}

async fn shutdown_task(state: Arc<Mutex<AppState>>) {
  let mut interval = interval(FRAME_DURATION);

  loop {
    interval.tick().await;
    let state = state.lock().await;
    if state.shutdown {
      break;
    }
  }
}
