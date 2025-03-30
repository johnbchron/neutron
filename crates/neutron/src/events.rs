use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use tokio::sync::mpsc::{self};

use crate::commands::Command;

pub async fn event_handler(commands: mpsc::Sender<Command>) {
  let mut event_stream = crossterm::event::EventStream::new();

  loop {
    if let Some(Ok(event)) = event_stream.next().await {
      let _ = match event {
        Event::Key(
          KeyEvent {
            code: KeyCode::Char('q') | KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            ..
          }
          | KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            ..
          },
        ) => commands.send(Command::Exit).await,
        _ => Ok(()),
      };
    }
  }
}
