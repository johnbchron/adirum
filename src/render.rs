pub mod camera;
pub mod render_buffer;
pub mod shapes;

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use ratatui::buffer::Cell;

use self::{
  camera::{Camera, MainCamera, RenderBuffer, update_camera_matrices},
  render_buffer::{RenderBufferSize, dummy_render, prepare_for_frame},
};
use crate::colors::{BASE_COLOR_RATATUI, PUNCHY_TEXT_COLOR_RATATUI};

const DEFAULT_CELL: Cell = const {
  let mut cell = Cell::EMPTY;
  cell.bg = BASE_COLOR_RATATUI;
  cell.fg = PUNCHY_TEXT_COLOR_RATATUI;
  cell
};

fn setup_camera(mut commands: Commands) {
  commands.spawn((
    Camera::default(),
    Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    MainCamera,
  ));
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Render;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
  fn build(&self, app: &mut App) {
    app.init_schedule(Render);
    app
      .world_mut()
      .resource_mut::<MainScheduleOrder>()
      .insert_after(PostUpdate, Render);

    app
      .init_resource::<RenderBuffer>()
      .init_resource::<RenderBufferSize>()
      .add_systems(Startup, setup_camera)
      .add_systems(PreUpdate, prepare_for_frame)
      .add_systems(PostUpdate, update_camera_matrices)
      .add_systems(Render, dummy_render);
  }
}
