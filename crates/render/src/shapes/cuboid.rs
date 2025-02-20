use std::f32::consts::PI;

use bevy::prelude::*;

use super::{
  CanvasArgs, DrawnShape, LineStyle, LineVariant, Material, PlaneArgs,
  PlaneStyle, ShapeBuffer, line::LineArgs,
};

pub struct CuboidArgs {
  pub half_extents: Vec3,
  pub style:        CuboidStyle,
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
      material:     self.line_material.clone(),
      cap_material: self.corner_material.clone(),
      variant:      self.line_variant,
    }
  }
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
      style: style.line_style(),
    });

    if let Some(face_material) = style.face_material.clone() {
      let mut plane_args = PlaneArgs {
        xy_half_extents: Vec2::ZERO,
        exclude_borders: true,
        style:           PlaneStyle {
          material: face_material,
        },
      };

      // front plane
      plane_args.xy_half_extents = halves.xy();
      let plane_transform =
        transform.mul_transform(Transform::from_xyz(0.0, 0.0, halves.y));
      plane_args.draw(buffer, args, &plane_transform);

      // back plane
      let plane_transform =
        transform.mul_transform(Transform::from_xyz(0.0, 0.0, -halves.y));
      plane_args.draw(buffer, args, &plane_transform);

      // left plane
      plane_args.xy_half_extents = halves.xz();
      let plane_transform = transform.mul_transform(
        Transform::from_xyz(-halves.x, 0.0, 0.0)
          .with_rotation(Quat::from_axis_angle(Vec3::Y, PI / 2.0)),
      );
      plane_args.draw(buffer, args, &plane_transform);

      // right plane
      let plane_transform = transform.mul_transform(
        Transform::from_xyz(halves.x, 0.0, 0.0)
          .with_rotation(Quat::from_axis_angle(Vec3::Y, PI / 2.0)),
      );
      plane_args.draw(buffer, args, &plane_transform);

      // top plane
      plane_args.xy_half_extents = halves.xz();
      let plane_transform = transform.mul_transform(
        Transform::from_xyz(0.0, halves.y, 0.0)
          .with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0)),
      );
      plane_args.draw(buffer, args, &plane_transform);

      // bottom plane
      plane_args.xy_half_extents = halves.xz();
      let plane_transform = transform.mul_transform(
        Transform::from_xyz(0.0, -halves.y, 0.0)
          .with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0)),
      );
      plane_args.draw(buffer, args, &plane_transform);
    }

    for line in lines {
      line.draw(buffer, args, transform);
    }
  }
}
