mod shape_buffer;

use bevy::prelude::*;
use ratatui::{buffer::Cell, prelude::Color};

pub use self::shape_buffer::ShapeBuffer;
use crate::render::{camera::CameraMatrix, render_buffer::RenderBufferSize};

pub enum Shape {
  Line(LineArgs),
  Cuboid(CuboidArgs),
}

pub struct LineArgs {
  pub from:  Vec3,
  pub to:    Vec3,
  pub style: LineStyle,
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
  pub fn world_to_canvas_coords(&self, point: Vec3) -> ((i32, i32), f32) {
    let ndc = self.camera_matrix.world_to_ndc(point);
    (
      self.render_buffer_size.ndc_to_canvas_coords(ndc.xy()),
      ndc.z,
    )
  }
  pub fn canvas_to_ndc_coords(&self, (x, y): (i32, i32), depth: f32) -> Vec3 {
    let ndc = self.render_buffer_size.canvas_to_ndc_coords((x, y));
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

impl DrawnShape for LineArgs {
  fn draw(&self, buffer: &mut ShapeBuffer, args: &CanvasArgs) {
    let LineArgs { from, to, style } = self;

    let canvas_from = args.world_to_canvas_coords(*from);
    let canvas_to = args.world_to_canvas_coords(*to);

    // // the angle of the line as it appears on the canvas
    // let angle = Vec2::new(
    //   (canvas_to.0.0 - canvas_from.0.0) as f32,
    //   (canvas_to.0.1 - canvas_from.0.1) as f32,
    // )
    // .angle_to(Vec2::Y)
    // .to_degrees();

    let points = match style {
      LineStyle::Thin { .. } => basic_line_draw(canvas_from, canvas_to),
    };

    for (i, ((x, y), t)) in points.iter().enumerate() {
      // get the angle between the next point and the previous point
      let next_point_index = (i + 1).min(points.len() - 1);
      let next_point = points[next_point_index].0;
      let prev_point = points[i.saturating_sub(1)].0;
      let angle = Vec2::new(
        (next_point.0 - prev_point.0) as f32,
        (next_point.1 - prev_point.1) as f32,
      )
      .angle_to(Vec2::X)
      .to_degrees();

      let cell = match style {
        LineStyle::Thin { fg, bg, cap } => {
          let mut cell = if i == 0 || i == points.len() - 1 {
            match cap {
              LineCap::Plus => Cell::new("+"),
            }
          } else {
            Cell::new(angle_to_cell(angle))
          };
          cell.fg = *fg;
          if let Some(bg) = bg {
            cell.bg = *bg;
          }

          cell
        }
      };

      if *x < 0
        || *y < 0
        || *x >= buffer.area().width as i32
        || *y >= buffer.area().height as i32
      {
        continue;
      }

      buffer.set(*x as _, *y as _, cell, *t);
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
    22.5..=67.5 => "/",
    67.5..=112.5 => "|",
    112.5..=157.5 => "\\",
    157.5..=202.5 => "-",
    202.5..=247.5 => "/",
    247.5..=292.5 => "|",
    292.5..=337.5 => "\\",
    337.5..=360.0 => "-",
    _ => unreachable!(),
  }
}

fn basic_line_draw(
  ((mut x1, mut y1), mut depth1): ((i32, i32), f32),
  ((x2, y2), depth2): ((i32, i32), f32),
) -> Vec<((i32, i32), f32)> {
  let mut points = Vec::new();

  let dx = (x2 - x1).abs();
  let dy = -(y2 - y1).abs();
  let sx = if x1 < x2 { 1 } else { -1 };
  let sy = if y1 < y2 { 1 } else { -1 };
  let mut err = dx + dy;

  loop {
    points.push(((x1, y1), depth1));

    if x1 == x2 && y1 == y2 {
      break;
    }

    let e2 = 2 * err;

    if e2 >= dy {
      err += dy;
      x1 += sx;
    }

    if e2 <= dx {
      err += dx;
      y1 += sy;
    }

    depth1 += (depth2 - depth1) / (dx + dy) as f32;
  }

  points
}
