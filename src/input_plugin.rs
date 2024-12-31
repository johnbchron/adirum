use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;

#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, keyboard_input_system);
  }
}

fn keyboard_input_system(
  mut events: EventReader<KeyEvent>,
  mut exit: EventWriter<AppExit>,
) {
  use crossterm::event::KeyCode;
  for event in events.read() {
    match event.code {
      KeyCode::Char('q') | KeyCode::Esc => {
        exit.send_default();
      }
      _ => {}
    }
  }
}
