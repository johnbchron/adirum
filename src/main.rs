mod colors;
mod input_plugin;
mod message;
mod render;
mod ui;

use std::time::Duration;

use bevy::{
  app::ScheduleRunnerPlugin,
  diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
  prelude::*,
};
use bevy_ratatui::RatatuiPlugins;

pub use self::colors::*;
use self::{
  input_plugin::InputPlugin, message::MessagePlugin,
  render::camera::CameraPlugin, ui::UiPlugin,
};

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
struct BlockCoords(IVec3);

#[derive(Component)]
enum StationBlock {
  Corridor,
  Room,
}

fn setup(mut commands: Commands) {
  commands.spawn((BlockCoords(IVec3::new(0, 0, 0)), StationBlock::Corridor));
  commands.spawn((BlockCoords(IVec3::new(0, 1, 0)), StationBlock::Room));
}

fn main() {
  let frame_period = Duration::from_secs_f64(1.0 / 120.0);
  App::new()
    .add_plugins(
      MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_period)),
    )
    .add_plugins((DiagnosticsPlugin, FrameTimeDiagnosticsPlugin))
    .add_plugins(RatatuiPlugins {
      enable_input_forwarding: true,
      ..default()
    })
    .add_plugins((InputPlugin, UiPlugin, MessagePlugin, CameraPlugin))
    .add_systems(Startup, setup)
    .run();
}
