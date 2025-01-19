mod cuboid;
mod line;
mod material;
mod shape_buffer;
mod thin_neighbor;

use bevy::prelude::*;

use self::line::LineArgs;
pub use self::{
  cuboid::CuboidArgs,
  material::{
    DrawnMaterial, Material, MaterialDrawRequest, MaterialDrawRequestType,
  },
  shape_buffer::ShapeBuffer,
};
use crate::render::{camera::CameraMatrix, render_buffer::RenderBufferSize};

pub enum Shape {
  Line(LineArgs),
  Cuboid(CuboidArgs),
}

#[derive(Clone)]
pub struct LineStyle {
  pub material:     Material,
  pub cap_material: Option<Material>,
  pub variant:      LineVariant,
}

#[derive(Clone)]
pub struct CuboidStyle {
  pub line_material:   Material,
  pub corner_material: Option<Material>,
  pub face_material:   Option<Material>,
  pub line_variant:    LineVariant,
}

impl CuboidStyle {
  fn line_style(&self) -> LineStyle {
    LineStyle {
      material:     self.line_material,
      cap_material: self.corner_material,
      variant:      self.line_variant,
    }
  }
}

#[derive(Clone, Copy)]
pub enum LineVariant {
  Thin,
}

pub struct CanvasArgs<'a> {
  camera_matrix:      &'a CameraMatrix,
  render_buffer_size: &'a RenderBufferSize,
}

impl<'a> CanvasArgs<'a> {
  pub fn new(
    camera_matrix: &'a CameraMatrix,
    render_buffer_size: &'a RenderBufferSize,
  ) -> Self {
    Self {
      camera_matrix,
      render_buffer_size,
    }
  }
  pub fn world_to_canvas_coords(&self, point: Vec3) -> (IVec2, f32) {
    let ndc = self.camera_matrix.world_to_ndc(point);
    (
      self.render_buffer_size.ndc_to_canvas_coords(ndc.xy()),
      ndc.z,
    )
  }
  pub fn canvas_to_ndc_coords(&self, point: IVec2, depth: f32) -> Vec3 {
    let ndc = self.render_buffer_size.canvas_to_ndc_coords(point);
    Vec3::new(ndc.x, ndc.y, depth)
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

impl DrawnShape for Shape {
  fn draw(
    &self,
    buffer: &mut ShapeBuffer,
    args: &CanvasArgs,
    transform: &Transform,
  ) {
    match self {
      Shape::Line(line) => line.draw(buffer, args, transform),
      Shape::Cuboid(cuboid) => cuboid.draw(buffer, args, transform),
    }
  }
}
