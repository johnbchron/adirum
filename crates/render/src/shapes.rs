mod circle;
mod cuboid;
mod line;
mod material;
mod plane;
mod polyline;
mod shape_buffer;
mod sign;
mod thin_neighbor;

use bevy::{ecs::system::SystemParam, prelude::*};

pub use self::{
  circle::*, cuboid::*, line::*, material::*, plane::*, polyline::*,
  shape_buffer::*, sign::*,
};
use super::camera::MainCameraMatrix;
use crate::render_buffer::RenderBufferSize;

#[derive(Component, Default)]
pub struct RenderedShape(ShapeBuffer);

impl RenderedShape {
  pub fn inner_mut(&mut self) -> &mut ShapeBuffer { &mut self.0 }
}

#[derive(Copy, Clone, Debug)]
pub struct ProjectedPoint(IVec2, f32);

#[derive(SystemParam)]
pub struct CanvasArgs<'w> {
  camera_matrix:      Res<'w, MainCameraMatrix>,
  render_buffer_size: Res<'w, RenderBufferSize>,
}

impl Clone for CanvasArgs<'_> {
  fn clone(&self) -> Self {
    CanvasArgs {
      camera_matrix:      Res::clone(&self.camera_matrix),
      render_buffer_size: Res::clone(&self.render_buffer_size),
    }
  }
}

impl CanvasArgs<'_> {
  pub fn world_to_canvas_coords(&self, point: Vec3) -> ProjectedPoint {
    let ndc = self.camera_matrix.world_to_ndc(point);
    ProjectedPoint(
      self.render_buffer_size.ndc_to_canvas_coords(ndc.xy()),
      ndc.z,
    )
  }

  #[allow(dead_code)]
  pub fn canvas_to_ndc_coords(&self, point: ProjectedPoint) -> Vec3 {
    let ndc = self.render_buffer_size.canvas_to_ndc_coords(point.0);
    Vec3::new(ndc.x, ndc.y, point.1)
  }

  pub fn character_aspect_ratio(&self) -> f32 {
    self.camera_matrix.character_aspect_ratio()
  }
}

pub trait DrawnShape {
  fn draw(
    &self,
    buffer: &mut ShapeBuffer,
    args: &CanvasArgs,
    transform: &Transform,
  );
}
