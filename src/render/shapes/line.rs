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
      LineStyle::Thin { .. } => basic_line_draw(canvas_from, canvas_to),
    };

    for (i, ((x, y), t)) in points.iter().enumerate() {
      // get the angle between the next point and the previous point
      let next_point_index = (i + 1).min(points.len() - 1);
      let next_point = points[next_point_index].0;
      let prev_point = points[i.saturating_sub(1)].0;
      let next_point_offset = (next_point.0 - *x, next_point.1 - *y);
      let prev_point_offset = (prev_point.0 - *x, prev_point.1 - *y);

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
