use bevy::prelude::*;
use ratatui::buffer::Cell;

use super::{
  CanvasArgs, DrawnShape, LineCap, LineStyle, ShapeBuffer,
  thin_neighbor::{Neighbor, thin_neighbor_symbol},
};

pub struct LineArgs {
  pub from:  Vec3,
  pub to:    Vec3,
  pub style: LineStyle,
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
      LineStyle::Thin { .. } => basic_8_connected(canvas_from, canvas_to),
    };

    for (i, (point, t)) in points.iter().enumerate() {
      // get the angle between the next point and the previous point
      let next_point_index = (i + 1).min(points.len() - 1);
      let next_point = points[next_point_index].0;
      let prev_point = points[i.saturating_sub(1)].0;
      let next_point_offset = next_point - point;
      let prev_point_offset = prev_point - point;

      let cell = match style {
        LineStyle::Thin { fg, bg, cap } => {
          let mut cell = if i == 0 || i == points.len() - 1 {
            match cap {
              LineCap::Plus => Cell::new("+"),
            }
          } else {
            Cell::new(thin_neighbor_symbol(
              Neighbor::find(prev_point_offset),
              Neighbor::find(next_point_offset),
            ))
          };
          cell.fg = *fg;
          if let Some(bg) = bg {
            cell.bg = *bg;
          }

          cell
        }
      };

      if point.x < 0
        || point.y < 0
        || point.x >= buffer.area().width as i32
        || point.y >= buffer.area().height as i32
      {
        continue;
      }

      buffer.set(point.x as _, point.y as _, cell, *t);
    }
  }
}

fn basic_8_connected(
  (p1, mut depth1): (IVec2, f32),
  (p2, depth2): (IVec2, f32),
) -> Vec<(IVec2, f32)> {
  let mut result = Vec::new();

  let delta = (p2 - p1).abs();
  let mut current = p1;
  let steps = delta.x.max(delta.y);

  let depth_step = (depth2 - depth1) / steps as f32;

  result.push((current, depth1));

  let step_x = if p1.x < p2.x {
    1
  } else if p1.x > p2.x {
    -1
  } else {
    0
  };
  let step_y = if p1.y < p2.y {
    1
  } else if p1.y > p2.y {
    -1
  } else {
    0
  };

  let mut err = delta.x - delta.y;

  for _ in 0..steps {
    let err2 = 2 * err;
    if err2 > -delta.y {
      err -= delta.y;
      current.x += step_x;
    }
    if err2 < delta.x {
      err += delta.x;
      current.y += step_y;
    }

    depth1 += depth_step;
    result.push((current, depth1));
  }

  result
}
