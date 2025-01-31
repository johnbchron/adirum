pub mod camera;
pub mod render_buffer;
pub mod shapes;

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};
use colors::{BASE_COLOR_RATATUI, PUNCHY_TEXT_COLOR_RATATUI};
use ratatui::buffer::Cell;

use self::{
  camera::{MainCameraMatrix, RenderBuffer, update_camera_matrices},
  render_buffer::{RenderBufferSize, prepare_for_frame},
  shapes::ShapeBuffer,
};

const DEFAULT_CELL: Cell = const {
  let mut cell = Cell::EMPTY;
  cell.bg = BASE_COLOR_RATATUI;
  cell.fg = PUNCHY_TEXT_COLOR_RATATUI;
  cell
};
const MAX_PROJECTED_DEPTH: f32 = 1000.0;

pub fn render_shape_buffers(
  mut render_buffer: ResMut<RenderBuffer>,
  mut query: Query<&mut shapes::RenderedShape>,
) {
  let master_shape_buffer =
    ShapeBuffer::merge(query.iter_mut().map(|b| b.into_inner().inner_mut()));
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
      .add_systems(PreUpdate, prepare_for_frame)
      .add_systems(PostUpdate, update_camera_matrices)
      .add_systems(Last, render_shape_buffers);
  }
}
