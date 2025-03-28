mod colors;

use std::{sync::Arc, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use ratatui::{
  Frame,
  style::Stylize,
  text::{Line, Text},
  widgets::{Block, Paragraph},
};
use tokio::{
  sync::{
    Mutex,
    mpsc::{self},
  },
  time::interval,
};

use self::colors::{BACKGROUND_COLOR_RATATUI, NORMAL_TEXT_COLOR_RATATUI};

const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 60);

struct ItemId(ulid::Ulid);

struct Item {
  id:   ItemId,
  deps: DependencySet,
}

enum DependencySet {
  All(Vec<ItemId>),
}

#[derive(Clone)]
struct AppState {
  shutdown: bool,
  counter:  usize,
}

enum Command {
  Exit,
  IncrementCounter(usize),
}

#[allow(clippy::derivable_impls)]
impl Default for AppState {
  fn default() -> Self {
    AppState {
      shutdown: false,
      counter:  0,
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
  let state = Arc::new(Mutex::new(AppState::default()));

  let (commands, command_rx) = mpsc::channel(100);

  tokio::select! {
    _ = tokio::spawn(draw_task(state.clone())) => {},
    _ = tokio::spawn(command_runner(state.clone(), command_rx)) => {},
    _ = tokio::spawn(event_handler(commands.clone())) => {},
    _ = tokio::spawn(shutdown_task(state.clone())) => {},
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

async fn event_handler(commands: mpsc::Sender<Command>) {
  let mut event_stream = crossterm::event::EventStream::new();

  loop {
    if let Some(Ok(event)) = event_stream.next().await {
      let _ = match event {
        Event::Key(KeyEvent {
          code: KeyCode::Char('q') | KeyCode::Esc,
          modifiers: KeyModifiers::NONE,
          kind: KeyEventKind::Press,
          ..
        }) => commands.send(Command::Exit).await,
        Event::Key(KeyEvent {
          code: KeyCode::Char(' '),
          modifiers: KeyModifiers::NONE,
          kind: KeyEventKind::Press,
          ..
        }) => commands.send(Command::IncrementCounter(1)).await,
        _ => Ok(()),
      };
    }
  }
}

async fn command_runner(
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
      Command::IncrementCounter(amount) => lock.counter += amount,
    }
  }
}

async fn draw_task(state: Arc<Mutex<AppState>>) {
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
