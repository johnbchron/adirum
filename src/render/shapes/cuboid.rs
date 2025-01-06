use bevy::prelude::*;

use super::{CanvasArgs, DrawnShape, LineStyle, ShapeBuffer, line::LineArgs};

pub struct CuboidArgs {
  pub origin:       Vec3,
  pub half_extents: Vec3,
  pub style:        LineStyle,
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
