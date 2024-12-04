use iced::{
  Theme,
  widget::{Column, column, text},
};

const APP_TITLE: &str = "Neutron";

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {}

#[derive(Debug, Default)]
struct App {
  value: i32,
}

impl App {
  pub fn view(&self) -> Column<AppMessage> {
    column![text(self.value).size(50)]
  }

  pub fn update(&mut self, message: AppMessage) { match message {} }

  pub fn theme(&self) -> Theme { Theme::TokyoNight }

  pub fn run() -> iced::Result {
    iced::application(APP_TITLE, Self::update, Self::view)
      .theme(Self::theme)
      .run()
  }
}

fn main() -> iced::Result { App::run() }
