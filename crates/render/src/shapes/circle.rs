use std::f32::consts::PI;

use bevy::prelude::*;

use super::{
  CanvasArgs, CircleStyle, DrawnShape, PolylineArgs, PolylineLoopStyle,
  PolylineStyle, ShapeBuffer,
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
    let estimated_canvas_perimeter = (x_radius + y_radius) * PI;

    let n_segments = (estimated_canvas_perimeter / 5.0).ceil() as usize;
    // let n_segments = 20;

    let iter = CircleSegmentIterator::new(*radius, n_segments);
    let points: Vec<_> = iter.collect();

    let polyline_args = PolylineArgs {
      points,
      style: PolylineStyle {
        material:   style.material,
        loop_style: PolylineLoopStyle::Closed {
          point_cap_material: None,
        },
      },
    };
    polyline_args.draw(buffer, args, transform);
  }
}

/// An iterator that takes world-space circle paramteters and returns
/// world-space
struct CircleSegmentIterator {
  radius:     f32,
  n_segments: usize,
  angle:      f32,
}

impl CircleSegmentIterator {
  fn new(radius: f32, n_segments: usize) -> CircleSegmentIterator {
    CircleSegmentIterator {
      radius,
      n_segments,
      angle: 0.0,
    }
  }
}

impl Iterator for CircleSegmentIterator {
  type Item = Vec3;

  fn next(&mut self) -> Option<Self::Item> {
    if self.angle >= 2.0 * PI {
      return None;
    }

    let x = self.angle.cos();
    let y = self.angle.sin();
    let point = Vec3::new(x * self.radius, y * self.radius, 0.0);
    self.angle += 2.0 * PI / self.n_segments as f32;

    Some(point)
  }
}
