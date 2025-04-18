use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};

use crate::app_state::AppState;

pub enum Command {
  Exit,
  SelectNavigateDown,
}

pub async fn command_runner(
  state: Arc<Mutex<AppState>>,
  mut command_rx: mpsc::Receiver<Command>,
) {
  loop {
    let Some(command) = command_rx.recv().await else {
      break;
    };

    let mut lock = state.lock().await;
    match command {
      Command::Exit => lock.shutdown = true,
      Command::SelectNavigateDown => todo!(),
    }
  }
}
