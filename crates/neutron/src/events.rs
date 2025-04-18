use std::sync::Arc;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use tokio::sync::{
  Mutex,
  mpsc::{self, error::SendError},
};

use crate::{
  app_state::{AppState, Screen},
  commands::{Command, SelectNavigateCommand},
};

type Commands = mpsc::Sender<Command>;
type CommandsResult = Result<(), SendError<Command>>;

pub async fn event_handler(
  state: Arc<Mutex<AppState>>,
  commands: Commands,
) -> CommandsResult {
  let mut event_stream = crossterm::event::EventStream::new();

  loop {
    let Some(Ok(event)) = event_stream.next().await else {
      continue;
    };

    detect_exit(&event, commands.clone()).await?;

    let state = state.lock().await;

    match &state.screen {
      Screen::ItemList(_) => {
        detect_navigate_selection(&event, commands.clone()).await?;
      }
      Screen::ItemGraph(_) => {
        detect_navigate_selection(&event, commands.clone()).await?;
      }
    }
  }
}

async fn detect_exit(event: &Event, commands: Commands) -> CommandsResult {
  // exit on CTRL-C
  if let Event::Key(KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
    kind: KeyEventKind::Press,
    ..
  })
  | Event::Key(KeyEvent {
    code: KeyCode::Char('q'),
    modifiers: KeyModifiers::NONE,
    kind: KeyEventKind::Press,
    ..
  }) = &event
  {
    commands.send(Command::Exit).await?;
  }

  Ok(())
}

async fn detect_navigate_selection(
  event: &Event,
  commands: Commands,
) -> CommandsResult {
  match event {
    Event::Key(KeyEvent {
      code: KeyCode::Down,
      modifiers: KeyModifiers::NONE,
      kind: KeyEventKind::Press | KeyEventKind::Repeat,
      ..
    }) => {
      commands
        .send(Command::SelectNavigate(SelectNavigateCommand::Down))
        .await?;
    }
    Event::Key(KeyEvent {
      code: KeyCode::Up,
      modifiers: KeyModifiers::NONE,
      kind: KeyEventKind::Press | KeyEventKind::Repeat,
      ..
    }) => {
      commands
        .send(Command::SelectNavigate(SelectNavigateCommand::Up))
        .await?;
    }
    Event::Key(KeyEvent {
      code: KeyCode::Esc,
      modifiers: KeyModifiers::NONE,
      kind: KeyEventKind::Press,
      ..
    }) => {
      commands
        .send(Command::SelectNavigate(SelectNavigateCommand::Escape))
        .await?;
    }
    _ => (),
  }

  Ok(())
}
