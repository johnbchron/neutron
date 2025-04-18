use std::{ops::DerefMut, sync::Arc};

use tokio::sync::{Mutex, mpsc};

use crate::app_state::{AppState, Screen};

pub enum SelectNavigateCommand {
  Down,
  Up,
  Escape,
}
pub enum Command {
  Exit,
  SelectNavigate(SelectNavigateCommand),
}

pub async fn command_runner(
  state: Arc<Mutex<AppState>>,
  mut command_rx: mpsc::Receiver<Command>,
) {
  loop {
    let Some(command) = command_rx.recv().await else {
      break;
    };

    match command {
      Command::Exit => {
        let mut state = state.lock().await;
        state.shutdown = true;
      }
      Command::SelectNavigate(sn_command) => {
        run_select_navigate(state.clone(), sn_command).await
      }
    }
  }
}

async fn run_select_navigate(
  state: Arc<Mutex<AppState>>,
  command: SelectNavigateCommand,
) {
  let mut lock = state.lock().await;
  let state: &mut AppState = lock.deref_mut();
  let AppState { items, screen, .. } = state;

  match command {
    SelectNavigateCommand::Down => match screen {
      Screen::ItemList(item_list_state) => {
        let sort_order = &item_list_state.sort_order;
        match item_list_state.selected {
          Some(current) => match items.after_item(sort_order, current) {
            Ok(Some(new_id)) => {
              item_list_state.selected = Some(new_id);
            }
            _ => {
              item_list_state.selected = items.first_item(sort_order);
            }
          },
          None => item_list_state.selected = items.first_item(sort_order),
        }
      }
      Screen::ItemGraph(_) => todo!(),
    },
    SelectNavigateCommand::Up => match screen {
      Screen::ItemList(item_list_state) => {
        let sort_order = &item_list_state.sort_order;
        match item_list_state.selected {
          Some(current) => match items.before_item(sort_order, current) {
            Ok(Some(new_id)) => {
              item_list_state.selected = Some(new_id);
            }
            _ => {
              item_list_state.selected = items.last_item(sort_order);
            }
          },
          None => item_list_state.selected = items.last_item(sort_order),
        }
      }
      Screen::ItemGraph(_) => todo!(),
    },
    SelectNavigateCommand::Escape => match screen {
      Screen::ItemList(item_list_state) => {
        item_list_state.selected = None;
      }
      Screen::ItemGraph(_) => todo!(),
    },
  }
}
