use bevy::prelude::*;

use super::{
  CanvasArgs, DrawnShape, Material, ShapeBuffer, line::basic_8_connected,
};
use crate::{
  MAX_PROJECTED_DEPTH,
  shapes::{MaterialDrawRequest, MaterialDrawRequestType, ProjectedPoint},
};

const PLANE_DEPTH_BIAS: f32 = 0.05 / MAX_PROJECTED_DEPTH;

pub struct PlaneArgs {
  pub xy_half_extents: Vec2,
  pub exclude_borders: bool,
  pub style:           PlaneStyle,
}

#[derive(Clone)]
pub struct PlaneStyle {
  pub material: Material,
}

const PLANE_POINTS: [Vec3; 4] = [
  Vec3::new(-1.0, 1.0, 0.0),
  Vec3::new(1.0, 1.0, 0.0),
  Vec3::new(1.0, -1.0, 0.0),
  Vec3::new(-1.0, -1.0, 0.0),
];

impl DrawnShape for PlaneArgs {
  fn draw(
    &self,
    buffer: &mut ShapeBuffer,
    args: &CanvasArgs,
    transform: &Transform,
  ) {
    let PlaneArgs {
      xy_half_extents,
      exclude_borders,
      style,
    } = self;

    let scaled_points =
      PLANE_POINTS.iter().map(|p| p * xy_half_extents.extend(1.0));

    let transformed_points =
      scaled_points.map(|p| transform.transform_point(p));

    let canvas_points = transformed_points
      .map(|p| args.world_to_canvas_coords(p))
      .collect::<Vec<_>>();

    // the left and right vertical lines (as if it had no transform)
    let mut left_vert_line =
      basic_8_connected(canvas_points[0], canvas_points[3]);
    let mut right_vert_line =
      basic_8_connected(canvas_points[1], canvas_points[2]);

    if *exclude_borders {
      // skips one on both sides by skip, reverse, skip, and then reverse
      left_vert_line = left_vert_line
        .into_iter()
        .skip(1)
        .rev()
        .skip(1)
        .rev()
        .collect();
      right_vert_line = right_vert_line
        .into_iter()
        .skip(1)
        .rev()
        .skip(1)
        .rev()
        .collect();
    }

    // make sure the left and right sides are the same length
    match left_vert_line.len() as i32 - right_vert_line.len() as i32 {
      // left is less than right
      i @ ..0 => {
        // copy the last point of the left line to make the list long enough
        if left_vert_line.is_empty() {
          return;
        }
        let last_point = *left_vert_line.last().unwrap();
        left_vert_line.extend(std::iter::repeat(last_point).take(-i as usize));
      }
      // equal
      0 => (),
      // left is greater than right
      i @ 1.. => {
        // copy the last point of the right line to make the list long enough
        if right_vert_line.is_empty() {
          return;
        }
        let last_point = *right_vert_line.last().unwrap();
        right_vert_line.extend(std::iter::repeat(last_point).take(i as usize));
      }
    };
    debug_assert_eq!(left_vert_line.len(), right_vert_line.len());

    let mut materials_to_draw = Vec::new();

    // for each pair of points on the right and left, draw a line across
    let left_to_right = left_vert_line.into_iter().zip(right_vert_line);
    for (left_point, right_point) in left_to_right {
      let mut row = basic_8_connected(left_point, right_point);

      if *exclude_borders {
        // skips one on both sides by skip, reverse, skip, and then reverse
        row = row.into_iter().skip(1).rev().skip(1).rev().collect();
      }

      for point in row {
        let mat_request_type = style.material.draw_request_type();
        let request = match mat_request_type {
          MaterialDrawRequestType::None => MaterialDrawRequest::None,
          MaterialDrawRequestType::Neighbors => {
            panic!("cannot draw sequenced material in plane (yet)")
          }
        };

        let biased_depth = point.depth() + PLANE_DEPTH_BIAS;
        let drawn_material = style.material.draw(request, biased_depth);

        materials_to_draw.push((
          drawn_material,
          ProjectedPoint::new(point.pos(), biased_depth),
        ));
      }
    }

    if materials_to_draw.is_empty() {
      return;
    }

    // deduplicate points_to_draw by position
    let min_x = materials_to_draw.iter().map(|p| p.1.pos().x).min().unwrap();
    let min_y = materials_to_draw.iter().map(|p| p.1.pos().y).min().unwrap();
    let max_x = materials_to_draw.iter().map(|p| p.1.pos().x).max().unwrap();
    materials_to_draw.sort_unstable_by_key(|p| {
      (p.1.pos().y - min_y) * (max_x - min_x) + (p.1.pos().x - min_x)
    });
    materials_to_draw.dedup_by_key(|p| p.1.pos());

    for point in materials_to_draw {
      buffer.draw(point.0, point.1);
    }
  }
}

// fn simple_line(
//   (p1, depth1): (IVec2, f32),
//   (p2, depth2): (IVec2, f32),
// ) -> Vec<(IVec2, f32)> {
//   let dx = p2.x as f32 - p1.x as f32;
//   let dy = p2.y as f32 - p1.y as f32;

//   let steps = dx.abs().max(dy.abs()).round() as usize;
//   let mut points = Vec::with_capacity(steps);

//   let x_step = dx / steps as f32;
//   let y_step = dy / steps as f32;

//   let mut x = p1.x as f32;
//   let mut y = p1.y as f32;

//   for i in 0..=steps {
//     let t = i as f32 / (steps) as f32;

//     let depth = depth1 + (depth2 - depth1) * t;
//     points.push((IVec2::new(x as _, y as _), depth));

//     x += x_step;
//     y += y_step;
//   }

//   points
// }
