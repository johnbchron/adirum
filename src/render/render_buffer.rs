use bevy::prelude::*;
use ratatui::prelude::Rect;

use super::camera::CameraMatrix;
use crate::{
  render::{Material, camera::MainCamera},
  ui::RenderedWidgetState,
};

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
    self
      .widget_state
      .buffer_mut()
      .content
      .fill(Material::Nothing.to_cell());
  }

  /// Draws a material at the given canvas coordinates.
  ///
  /// `(0, 0)` is the top-left corner of the canvas, and `(width - 1, height -
  /// 1)` is the bottom-right corner.
  pub fn draw_in_canvas_coords(
    &mut self,
    (x, y): (u16, u16),
    material: Material,
  ) {
    let area = self.render_area();

    if x >= area.width || y >= area.height {
      return;
    }

    let index = (y * area.width + x) as usize;
    self.widget_state.buffer_mut().content[index] = material.to_cell();
  }

  /// Draws a material at the given normalized device coordinates.
  ///
  /// `(-1, -1)` is the bottom-left corner of the screen, and `(1, 1)` is the
  /// top-right corner.
  pub fn draw_in_ndc_coords(&mut self, point: Vec2, material: Material) {
    let area = self.render_area();
    let x = ((point.x + 1.0) * 0.5 * area.width as f32) as u16;
    let y = ((-point.y + 1.0) * 0.5 * area.height as f32) as u16;

    self.draw_in_canvas_coords((x, y), material);
  }

  /// Draws a material at the given world coordinates.
  pub fn draw_in_world_coords(&mut self, point: Vec3, material: Material) {
    let point = self.camera_matrix.world_to_ndc(point);
    self.draw_in_ndc_coords(Vec2::new(point.x, point.y), material);
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

pub fn dummy_render(mut camera_buffer: ResMut<RenderBuffer>) {
  use super::shapes::*;

  let line_style = LineStyle::Thin {
    cap: LineCap::Plus,
    fg:  Material::Wall.to_cell().fg,
    bg:  Some(Material::Wall.to_cell().bg),
  };
  let cuboid = CuboidArgs {
    origin:       Vec3::ZERO,
    half_extents: Vec3::splat(0.5),
    style:        line_style.clone(),
  };

  let camera_matrix = &camera_buffer.camera_matrix;
  let camera_buffer_area = camera_buffer.render_area();
  let render_buffer_size =
    &RenderBufferSize(camera_buffer_area.width, camera_buffer_area.height);

  let args = CanvasArgs::new(camera_matrix, render_buffer_size);

  let mut shape_buffer = ShapeBuffer::new(camera_buffer.render_area());

  cuboid.draw(&mut shape_buffer, &args);

  let mut shape_buffer = shape_buffer.into_buffer();
  let intersection = camera_buffer
    .widget_state
    .buffer_mut()
    .area()
    .intersection(*shape_buffer.area());
  shape_buffer.resize(intersection);

  camera_buffer.widget_state.buffer_mut().merge(&shape_buffer);

  // // draw the vertices of a cube
  // let size = 0.25;
  // let points = [
  //   Vec3::new(-size, -size, -size),
  //   Vec3::new(size, -size, -size),
  //   Vec3::new(size, size, -size),
  //   Vec3::new(-size, size, -size),
  //   Vec3::new(-size, -size, size),
  //   Vec3::new(size, -size, size),
  //   Vec3::new(size, size, size),
  //   Vec3::new(-size, size, size),
  // ];

  // let edges = [
  //   (0, 1),
  //   (1, 2),
  //   (2, 3),
  //   (3, 0),
  //   (4, 5),
  //   (5, 6),
  //   (6, 7),
  //   (7, 4),
  //   (0, 4),
  //   (1, 5),
  //   (2, 6),
  //   (3, 7),
  // ];

  // let segments = 20;

  // for (start, end) in edges.iter() {
  //   let start = points[*start];
  //   let end = points[*end];
  //   let delta = end - start;
  //   let length = delta.length();
  //   let direction = delta.normalize();

  //   for i in 0..segments {
  //     let t = i as f32 / segments as f32;
  //     let point = start + direction * t * length;
  //     camera_buffer.draw_in_world_coords(point, Material::Wall);
  //   }
  // }

  // camera_buffer.draw_in_world_coords(Vec3::ZERO, Material::Wall);
  // camera_buffer.draw_in_world_coords(Vec3::new(0.0, 1.0, 0.0),
  // Material::Wall);

  // // draw a circle in NDC
  // let radius = 0.5;
  // let center = Vec2::new(0.0, 0.0);
  // let segments = 200;
  // let step = 2.0 * std::f32::consts::PI / segments as f32;

  // for i in 0..segments {
  //   let angle = i as f32 * step;
  //   let x = radius * angle.cos();
  //   let y = radius * angle.sin();
  //   camera_buffer.draw_in_ndc_coords(center + Vec2::new(x, y),
  // Material::Wall); }

  // // draw a line in NDC
  // let start = Vec2::new(-0.5, -0.5);
  // let end = Vec2::new(0.5, 0.5);
  // let delta = end - start;
  // let length = delta.length();
  // let direction = delta.normalize();
  // let segments = 100;

  // for i in 0..segments {
  //   let t = i as f32 / segments as f32;
  //   let point = start + direction * t * length;
  //   camera_buffer.draw_in_ndc_coords(point, Material::Wall);
  // }
}
