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
use blocks::{BlockCoords, BlockPlugin, StationBlockType};
use ratatui::{
  style::{Color, Stylize},
  widgets::Wrap,
};
use render::{
  RenderPlugin,
  camera::{Camera, MainCamera},
  shapes::{CanvasArgs, RenderedShape},
};

use self::{input_plugin::InputPlugin, message::MessagePlugin, ui::UiPlugin};

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
    BlockCoords::new_single(IVec3::new(0, 0, 0)),
    StationBlockType::Room,
  ));
  commands.spawn((
    BlockCoords::new_single(IVec3::new(1, 0, 0)),
    StationBlockType::Room,
  ));

  commands.spawn(SignTest);
}

#[derive(Component)]
#[require(RenderedShape, Transform)]
pub struct SignTest;

fn render_sign_test(
  mut query: Query<(&Transform, &mut RenderedShape), With<SignTest>>,
  canvas_args: CanvasArgs,
) {
  use render::shapes::*;

  for (transform, mut buffer) in query.iter_mut() {
    let paragraph = ratatui::widgets::Paragraph::new(
      "Hello, world! This is a test sign. It's a very long sign, so it's \
       going to wrap around to the next line. I have to just keep writing \
       nonsense so I can test if the wrapping works.",
    )
    .wrap(Wrap { trim: false })
    .fg(Color::Rgb(255, 255, 255))
    .bg(Color::Rgb(40, 0, 0));

    let text = SignArgs {
      content:    paragraph,
      max_width:  32,
      max_height: None,
      position:   Vec3::ZERO,
      anchor:     Vec2::NEG_ONE,
      on_top:     true,
    };

    text.draw(buffer.inner_mut(), &canvas_args, transform);
  }
}

fn main() {
  #[cfg(feature = "no-vsync")]
  let frame_period = Duration::from_secs_f64(0.0);
  #[cfg(not(feature = "no-vsync"))]
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
      BlockPlugin,
      InputPlugin,
      MessagePlugin,
      RenderPlugin,
      UiPlugin,
    ))
    .add_systems(Startup, setup_station_blocks)
    .add_systems(Startup, setup_camera)
    .add_systems(Update, render_sign_test)
    .run();
}
