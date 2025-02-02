use std::ops::Rem;

use bevy::prelude::*;

use super::{
  CanvasArgs, CircleStyle, DrawnShape, MaterialDrawRequest,
  MaterialDrawRequestType, ShapeBuffer, thin_neighbor::Neighbor,
};

pub struct CircleArgs {
  pub radius: f32,
  pub style:  CircleStyle,
}

impl DrawnShape for CircleArgs {
  fn draw(
    &self,
    buffer: &mut ShapeBuffer,
    args: &CanvasArgs,
    transform: &Transform,
  ) {
    let CircleArgs { radius, style } = self;

    let transformed_center = transform.transform_point(Vec3::ZERO);
    let canvas_center = args.world_to_canvas_coords(transformed_center);

    let canvas_positive_x = args.world_to_canvas_coords(
      transformed_center + Vec3::new(*radius, 0.0, 0.0),
    );
    let canvas_positive_y = args.world_to_canvas_coords(
      transformed_center + Vec3::new(0.0, *radius, 0.0),
    );

    // treat the transformed version like an ellipse and calculate perimeter
    let x_radius = (canvas_positive_x.0 - canvas_center.0).as_vec2().length();
    let y_radius = (canvas_positive_y.0 - canvas_center.0).as_vec2().length();
    let estimated_canvas_perimeter =
      (x_radius + y_radius) * std::f32::consts::PI;

    // estimate the angle step
    let estimated_angle_step =
      (2.0 * std::f32::consts::PI) / estimated_canvas_perimeter;
    let angle_step = estimated_angle_step * 1.5;

    let mut angle = 0.0;
    let mut points = Vec::new();
    let mut last_point: Option<(IVec2, f32)> = None;

    // iterate around the circle
    while angle <= 2.0 * std::f32::consts::PI {
      let local_point =
        Vec3::new(angle.cos() * *radius, angle.sin() * *radius, 0.0);
      let transformed_point = transform.transform_point(local_point);
      let canvas_point = args.world_to_canvas_coords(transformed_point);

      // add only if not a duplicate
      if let Some(last) = last_point {
        if last.0 != canvas_point.0 {
          points.push(canvas_point);
        }
      }

      last_point = Some(canvas_point);
      angle += angle_step;
    }

    if points.is_empty() {
      return;
    }

    // remove duplicate last point
    if points.len() > 1 && points[0] == points[points.len() - 1] {
      points.pop();
    }

    for (i, (position, proj_depth)) in points.iter().enumerate() {
      let next_point_index = (i + 2).rem(points.len());
      let next_point = points[next_point_index].0;
      let prev_point_index = (points.len() + i - 2).rem(points.len());
      let prev_point = points[prev_point_index].0;
      let next_point_offset = next_point - position;
      let prev_point_offset = prev_point - position;

      // figure out what info we need
      let mat_request_type = style.material.draw_request_type();

      // fill in the info
      let request = match mat_request_type {
        MaterialDrawRequestType::None => MaterialDrawRequest::None,
        MaterialDrawRequestType::Neighbors => MaterialDrawRequest::Neighbors {
          prev: Neighbor::find(
            prev_point_offset,
            args.character_aspect_ratio(),
          ),
          next: Neighbor::find(
            next_point_offset,
            args.character_aspect_ratio(),
          ),
        },
      };

      // determine the character
      let drawn_material = style.material.draw(request, *proj_depth);

      buffer.draw(drawn_material, *position, *proj_depth);
    }
  }
}
