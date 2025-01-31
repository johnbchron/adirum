pub mod camera;
pub mod render_buffer;
pub mod shapes;

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use ratatui::buffer::Cell;

use self::{
  camera::{
    Camera, MainCamera, MainCameraMatrix, RenderBuffer, update_camera_matrices,
  },
  render_buffer::{RenderBufferSize, prepare_for_frame},
  shapes::ShapeBuffer,
};
use crate::colors::{BASE_COLOR_RATATUI, PUNCHY_TEXT_COLOR_RATATUI};

const DEFAULT_CELL: Cell = const {
  let mut cell = Cell::EMPTY;
  cell.bg = BASE_COLOR_RATATUI;
  cell.fg = PUNCHY_TEXT_COLOR_RATATUI;
  cell
};
const MAX_PROJECTED_DEPTH: f32 = 1000.0;

fn setup_camera(mut commands: Commands) {
  commands.spawn((
    Camera::default().with_scale(0.2),
    Transform::from_xyz(-(10.0_f32.sqrt()) / 3.0, 10.0_f32.sqrt() / 3.0, 10.0)
      .looking_to(Vec3::NEG_Z, Vec3::Y),
    MainCamera,
  ));
}

fn render_shape_buffers(
  mut render_buffer: ResMut<RenderBuffer>,
  mut query: Query<&mut shapes::ShapeBuffer>,
) {
  let master_shape_buffer =
    ShapeBuffer::merge(query.iter_mut().map(|b| b.into_inner()));
  let truncated_master = master_shape_buffer.truncate();
  let rendered_master = truncated_master.render(render_buffer.render_area());

  render_buffer
    .widget_state_mut()
    .buffer_mut()
    .merge(&rendered_master);
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
      .init_resource::<MainCameraMatrix>()
      .add_systems(Startup, setup_camera)
      .add_systems(PreUpdate, prepare_for_frame)
      .add_systems(PostUpdate, update_camera_matrices)
      .add_systems(Last, render_shape_buffers.before(crate::ui::draw_ui));
    // .add_systems(Render, dummy_render);
  }
}
