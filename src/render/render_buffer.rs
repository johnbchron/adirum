use std::f32::consts::PI;

use bevy::prelude::*;
use ratatui::prelude::Rect;

use super::{DEFAULT_CELL, camera::CameraMatrix};
use crate::{render::camera::MainCamera, ui::RenderedWidgetState};

#[derive(Resource, Default)]
pub struct RenderBufferSize(pub u16, pub u16);

impl RenderBufferSize {
  pub fn ndc_to_canvas_coords(&self, point: Vec2) -> IVec2 {
    let x: f32 = (point.x + 1.0) / 2.0 * self.0 as f32;
    let y: f32 = (-point.y + 1.0) / 2.0 * self.1 as f32;
    IVec2::new(x.floor() as i32, y.floor() as i32)
  }

  pub fn canvas_to_ndc_coords(&self, point: IVec2) -> Vec2 {
    let x = point.x as f32 / self.0 as f32 * 2.0 - 1.0;
    let y = -point.y as f32 / self.1 as f32 * 2.0 - 1.0;
    Vec2::new(x, y)
  }
}

#[derive(Resource)]
pub struct RenderBuffer {
  camera_matrix: CameraMatrix,
  widget_state:  RenderedWidgetState,
}

impl RenderBuffer {
  pub fn new(camera_matrix: CameraMatrix) -> Self {
    Self {
      camera_matrix,
      widget_state: RenderedWidgetState::new(),
    }
  }

  pub fn widget_state_mut(&mut self) -> &mut RenderedWidgetState {
    &mut self.widget_state
  }

  fn render_area(&self) -> Rect { self.widget_state.last_area() }

  /// Clears the render buffer and resizes it based off the area in the widget
  /// state.
  fn update_render_buffer_size(&mut self) {
    let area = self.render_area();

    self.widget_state.buffer_mut().resize(area);
    self.widget_state.buffer_mut().content.fill(DEFAULT_CELL);
  }
}

impl Default for RenderBuffer {
  fn default() -> Self { Self::new(CameraMatrix::default()) }
}

pub fn prepare_for_frame(
  mut render_buffer_size: ResMut<RenderBufferSize>,
  mut render_buffer: ResMut<RenderBuffer>,
  camera_matrix: Query<&CameraMatrix, With<MainCamera>>,
) {
  // propagate render area to `RenderBufferSize`
  let area = render_buffer.render_area();
  render_buffer_size.0 = area.width;
  render_buffer_size.1 = area.height;

  // resize the render buffer to what the widget used last
  render_buffer.update_render_buffer_size();

  // store the camera matrix from the main camera
  if let Ok(main_camera_matrix) = camera_matrix.get_single() {
    render_buffer.camera_matrix.clone_from(main_camera_matrix);
  }
}

pub fn dummy_render(mut camera_buffer: ResMut<RenderBuffer>, time: Res<Time>) {
  use super::shapes::*;

  let cuboid_style = CuboidStyle {
    line_material:   Material::WallEdge,
    corner_material: Some(Material::WallCorner),
    face_material:   Some(Material::WallFace),
    line_variant:    LineVariant::Thin,
  };
  let cuboid = CuboidArgs {
    half_extents: Vec3::splat(0.5),
    style:        cuboid_style,
  };

  let camera_matrix = &camera_buffer.camera_matrix;
  let camera_buffer_area = camera_buffer.render_area();
  let render_buffer_size =
    &RenderBufferSize(camera_buffer_area.width, camera_buffer_area.height);

  let args = CanvasArgs::new(camera_matrix, render_buffer_size);

  let mut shape_buffer = ShapeBuffer::new();

  let mut transform = Transform::IDENTITY.with_translation(Vec3::NEG_Y);
  // transform.rotate_x(PI * 2.0 * time.elapsed_secs() / 2.0);
  // transform.rotate_y(PI * 2.0 * time.elapsed_secs() / 5.0);
  // transform.rotate_z(PI * 2.0 * time.elapsed_secs() / 8.0);
  cuboid.draw(&mut shape_buffer, &args, &transform);

  let truncated_buffer = shape_buffer.truncate();
  let rendered_buffer =
    truncated_buffer.render(camera_buffer.widget_state.buffer_mut().area);

  camera_buffer
    .widget_state
    .buffer_mut()
    .merge(&rendered_buffer);
}
