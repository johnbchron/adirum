mod colors;
mod input_plugin;
mod message;
mod ui;

use std::time::Duration;

use bevy::{
  app::ScheduleRunnerPlugin,
  diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
  prelude::*,
};
use bevy_ratatui::RatatuiPlugins;

pub use self::colors::*;
use self::{input_plugin::InputPlugin, message::MessagePlugin, ui::UiPlugin};

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
struct BlockCoords(i32, i32);

#[derive(Component)]
enum StationBlock {
  Corridor,
  Room,
}

/// Map view settings.
#[derive(Resource)]
struct MapView {
  center: Vec2,
  zoom:   u8,
}

impl Default for MapView {
  fn default() -> Self {
    MapView {
      center: Vec2::new(0.0, 0.0),
      zoom:   1,
    }
  }
}

/// The coordinate system for the station.
#[derive(Resource)]
struct StationCoordSystem {
  /// Block side length in meters.
  block_size: f32,
  /// Location of block (0, 0) in worldspace.
  origin:     Vec2,
}

impl Default for StationCoordSystem {
  fn default() -> Self {
    StationCoordSystem {
      block_size: 2.0,
      origin:     Vec2::new(0.0, 0.0),
    }
  }
}

fn setup(mut commands: Commands) {
  commands.spawn((BlockCoords(0, 0), StationBlock::Corridor));
}

fn main() {
  let frame_period = Duration::from_secs_f64(1.0 / 60.0);
  App::new()
    .add_plugins(
      MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_period)),
    )
    .add_plugins((DiagnosticsPlugin, FrameTimeDiagnosticsPlugin))
    .add_plugins(RatatuiPlugins::default())
    .add_plugins((InputPlugin, UiPlugin, MessagePlugin))
    .add_systems(Startup, setup)
    .run();
}
