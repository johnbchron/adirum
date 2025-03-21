mod input_plugin;
mod ui;

use std::time::Duration;

use bevy::{
  app::ScheduleRunnerPlugin,
  diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
  prelude::*,
};
use bevy_ratatui::RatatuiPlugins;
use blocks::{BlockCoords, BlockPlugin, StationBlockType};
use message::MessagePlugin;
use render::{
  RenderPlugin,
  camera::{Camera, MainCamera},
  debug_signage::DebugSign,
};

use self::{input_plugin::InputPlugin, ui::UiPlugin};

fn setup_camera(mut commands: Commands) {
  commands.spawn((
    Camera::default().with_scale(0.2),
    Transform::from_xyz(-(10.0_f32.sqrt()) / 3.0, 10.0_f32.sqrt() / 3.0, 10.0)
      .looking_to(Vec3::NEG_Z, Vec3::Y),
    MainCamera,
  ));
}

fn setup_station_blocks(mut commands: Commands) {
  commands.spawn((
    BlockCoords::new(IVec3::new(0, 0, 0)),
    StationBlockType::QuadRoomXZ,
  ));
  commands.spawn((
    BlockCoords::new(IVec3::new(2, 0, 0)),
    StationBlockType::Room,
    DebugSign::default(),
  ));
}

fn main() {
  #[cfg(feature = "no-vsync")]
  let frame_period = Duration::from_secs_f64(0.0);
  #[cfg(not(feature = "no-vsync"))]
  let frame_period = Duration::from_secs_f64(1.0 / 60.0);

  App::new()
    .register_type::<Transform>()
    .add_plugins(
      MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_period)),
    )
    .add_plugins((
      TransformPlugin,
      HierarchyPlugin,
      DiagnosticsPlugin,
      FrameTimeDiagnosticsPlugin,
    ))
    .add_plugins(RatatuiPlugins {
      enable_input_forwarding: true,
      ..default()
    })
    .add_plugins((
      BlockPlugin,
      InputPlugin,
      MessagePlugin,
      RenderPlugin,
      UiPlugin,
    ))
    .add_systems(Startup, setup_station_blocks)
    .add_systems(Startup, setup_camera)
    .run();
}
