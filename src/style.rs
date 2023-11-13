use bevy::prelude::*;

pub enum ThemeColor {
  Text,
  Background,
  Foreground,
  Outline,
}

impl ThemeColor {
  pub fn color(&self) -> Color {
    match self {
      // colors are source from my current obsidian theme
      ThemeColor::Text => Color::rgb(0.85, 0.85, 0.85),
      ThemeColor::Background => Color::rgb(0.09, 0.11, 0.13),
      ThemeColor::Foreground => Color::rgb(0.11, 0.13, 0.15),
      ThemeColor::Outline => Color::rgb(0.21, 0.22, 0.25),
    }
  }
}
