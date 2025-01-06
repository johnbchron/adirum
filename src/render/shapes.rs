mod line;
mod shape_buffer;
mod thin_neighbor;

use bevy::prelude::*;
use ratatui::prelude::Color;

use self::line::LineArgs;
pub use self::shape_buffer::ShapeBuffer;
use crate::render::{camera::CameraMatrix, render_buffer::RenderBufferSize};

pub enum Shape {
  Line(LineArgs),
  Cuboid(CuboidArgs),
}

pub struct CuboidArgs {
  pub origin:       Vec3,
  pub half_extents: Vec3,
  pub style:        LineStyle,
}

#[derive(Clone)]
pub enum LineStyle {
  Thin {
    fg:  Color,
    bg:  Option<Color>,
    cap: LineCap,
  },
}

#[derive(Clone)]
pub enum LineCap {
  Plus,
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
  fn draw(&self, buffer: &mut ShapeBuffer, args: &CanvasArgs);
}

impl DrawnShape for Shape {
  fn draw(&self, buffer: &mut ShapeBuffer, args: &CanvasArgs) {
    match self {
      Shape::Line(line) => line.draw(buffer, args),
      Shape::Cuboid(cuboid) => cuboid.draw(buffer, args),
    }
  }
}

impl DrawnShape for CuboidArgs {
  fn draw(&self, buffer: &mut ShapeBuffer, args: &CanvasArgs) {
    let CuboidArgs {
      origin: o,
      half_extents: halves,
      style,
    } = self;

    let points = [
      Vec3::new(-1.0, -1.0, -1.0),
      Vec3::new(1.0, -1.0, -1.0),
      Vec3::new(1.0, 1.0, -1.0),
      Vec3::new(-1.0, 1.0, -1.0),
      Vec3::new(-1.0, -1.0, 1.0),
      Vec3::new(1.0, -1.0, 1.0),
      Vec3::new(1.0, 1.0, 1.0),
      Vec3::new(-1.0, 1.0, 1.0),
    ];

    let edges = [
      (0, 1),
      (1, 2),
      (2, 3),
      (3, 0),
      (4, 5),
      (5, 6),
      (6, 7),
      (7, 4),
      (0, 4),
      (1, 5),
      (2, 6),
      (3, 7),
    ];

    let world_points: Vec<Vec3> =
      points.iter().map(|p| o + p * halves).collect::<Vec<_>>();

    let lines = edges
      .iter()
      .map(|&(i, j)| LineArgs {
        from:  world_points[i],
        to:    world_points[j],
        style: style.clone(),
      })
      .collect::<Vec<_>>();

    for line in lines {
      line.draw(buffer, args);
    }
  }
}

/// Convert an angle (in ranges) to a cell character.
fn angle_to_cell(angle: f32) -> &'static str {
  match angle.rem_euclid(360.0) {
    0.0..=22.5 => "-",
    22.5..=67.5 => "╱",
    67.5..=112.5 => "|",
    112.5..=157.5 => "\\",
    157.5..=202.5 => "-",
    202.5..=247.5 => "╱",
    247.5..=292.5 => "|",
    292.5..=337.5 => "\\",
    337.5..=360.0 => "-",
    _ => unreachable!(),
  }
}
