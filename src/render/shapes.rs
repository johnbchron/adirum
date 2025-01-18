mod cuboid;
mod line;
mod shape_buffer;
mod thin_neighbor;

use bevy::prelude::*;
use ratatui::buffer::Cell;
use thin_neighbor::{Neighbor, thin_neighbor_symbol};

use self::line::LineArgs;
pub use self::{cuboid::CuboidArgs, shape_buffer::ShapeBuffer};
use crate::{
  colors::{
    BASE_COLOR_RATATUI, DIM_TEXT_COLOR_RATATUI, PUNCHY_TEXT_COLOR_RATATUI,
  },
  render::{camera::CameraMatrix, render_buffer::RenderBufferSize},
};

/// A material descriptor.
#[derive(Clone, Copy, Debug)]
pub enum Material {
  WallFace,
  WallEdge,
  WallCorner,
}

impl Material {
  pub fn draw_request_type(&self) -> MaterialDrawRequestType {
    match self {
      Material::WallFace => MaterialDrawRequestType::None,
      Material::WallEdge => MaterialDrawRequestType::Neighbors,
      Material::WallCorner => MaterialDrawRequestType::None,
    }
  }

  pub fn draw(&self, draw_request: MaterialDrawRequest) -> DrawnMaterial {
    match (self, draw_request) {
      (Material::WallFace, MaterialDrawRequest::None) => {
        DrawnMaterial(Material::WallFace, "")
      }
      (Material::WallEdge, MaterialDrawRequest::Neighbors { prev, next }) => {
        DrawnMaterial(Material::WallEdge, thin_neighbor_symbol(prev, next))
      }
      (Material::WallCorner, MaterialDrawRequest::None) => {
        DrawnMaterial(Material::WallCorner, "+")
      }
      (mat, req) => panic!(
        "material/draw_request mismatch: got material {mat:?}, draw_request \
         {req:?}"
      ),
    }
  }
}

/// The kind of information a given material variant needs to draw itself.
#[derive(Clone, Debug)]
pub enum MaterialDrawRequestType {
  /// No additional information needed.
  None,
  /// This material variant needs its next and previous neighbor directions.
  Neighbors,
}

/// The information a material variant needs to draw itself.
#[derive(Clone, Debug)]
pub enum MaterialDrawRequest {
  /// No additional information needed.
  None,
  /// The previous and next neighbor directions for this cell.
  Neighbors { prev: Neighbor, next: Neighbor },
}

/// A material whose stroke has been determined.
#[derive(Clone, Copy, Debug)]
pub struct DrawnMaterial(Material, &'static str);

impl DrawnMaterial {
  pub fn render(&self, behind: Option<&Self>) -> Cell {
    match self {
      DrawnMaterial(Material::WallFace, _) => match behind {
        Some(DrawnMaterial(Material::WallFace, _)) | None => {
          let mut cell = Cell::new(self.1);
          cell.set_bg(BASE_COLOR_RATATUI);
          cell
        }
        Some(DrawnMaterial(Material::WallEdge, sym)) => {
          let mut cell = Cell::new(sym);
          cell.set_bg(BASE_COLOR_RATATUI);
          cell.set_fg(DIM_TEXT_COLOR_RATATUI);
          cell
        }
        Some(DrawnMaterial(Material::WallCorner, sym)) => {
          let mut cell = Cell::new(sym);
          cell.set_bg(BASE_COLOR_RATATUI);
          cell.set_fg(DIM_TEXT_COLOR_RATATUI);
          cell
        }
      },
      DrawnMaterial(Material::WallEdge, sym) => {
        let mut cell = Cell::new(sym);
        cell.set_bg(BASE_COLOR_RATATUI);
        cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
        cell
      }
      DrawnMaterial(Material::WallCorner, sym) => {
        let mut cell = Cell::new(sym);
        cell.set_bg(BASE_COLOR_RATATUI);
        cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
        cell
      }
    }
  }
}

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
