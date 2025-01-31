mod block_coords;
mod input_plugin;
mod message;
mod render;
mod station_block;
mod ui;

use std::time::Duration;

use bevy::{
  app::ScheduleRunnerPlugin,
  diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
  prelude::*,
};
use bevy_ratatui::RatatuiPlugins;
use station_block::StationBlockPlugin;

use self::{
  block_coords::{BlockCoords, BlockCoordsPlugin},
  input_plugin::InputPlugin,
  message::MessagePlugin,
  render::RenderPlugin,
  station_block::StationBlock,
  ui::UiPlugin,
};

fn setup(mut commands: Commands) {
  commands.spawn((BlockCoords::new(IVec3::new(0, 0, 0)), StationBlock::Room));
  commands.spawn((BlockCoords::new(IVec3::new(1, 0, 0)), StationBlock::Room));
}

fn main() {
  let frame_period = Duration::from_secs_f64(1.0 / 60.0);

  App::new()
    .add_plugins(
      MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_period)),
    )
    .add_plugins((DiagnosticsPlugin, FrameTimeDiagnosticsPlugin))
    .add_plugins(RatatuiPlugins {
      enable_input_forwarding: true,
      ..default()
    })
    .add_plugins((
      BlockCoordsPlugin,
      InputPlugin,
      MessagePlugin,
      RenderPlugin,
      StationBlockPlugin,
      UiPlugin,
    ))
    .add_systems(Startup, setup)
    .run();
}
