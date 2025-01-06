use bevy::prelude::*;

use super::{
  CanvasArgs, CuboidStyle, DrawnShape, LineStyle, ShapeBuffer, line::LineArgs,
};

pub struct CuboidArgs {
  pub half_extents: Vec3,
  pub style:        CuboidStyle,
}

const CUBOID_POINTS: [Vec3; 8] = [
  Vec3::new(-1.0, -1.0, -1.0),
  Vec3::new(1.0, -1.0, -1.0),
  Vec3::new(1.0, 1.0, -1.0),
  Vec3::new(-1.0, 1.0, -1.0),
  Vec3::new(-1.0, -1.0, 1.0),
  Vec3::new(1.0, -1.0, 1.0),
  Vec3::new(1.0, 1.0, 1.0),
  Vec3::new(-1.0, 1.0, 1.0),
];

const CUBOID_EDGES: [(usize, usize); 12] = [
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

impl DrawnShape for CuboidArgs {
  fn draw(
    &self,
    buffer: &mut ShapeBuffer,
    args: &CanvasArgs,
    transform: &Transform,
  ) {
    let CuboidArgs {
      half_extents: halves,
      style,
    } = self;

    let scaled_points: Vec<Vec3> =
      CUBOID_POINTS.iter().map(|p| p * halves).collect::<Vec<_>>();

    let lines = CUBOID_EDGES.iter().map(|&(i, j)| LineArgs {
      from:  scaled_points[i],
      to:    scaled_points[j],
      style: style.line_style.clone(),
    });

    for line in lines {
      line.draw(buffer, args, transform);
    }
  }
}
