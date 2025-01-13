use std::cmp::Ordering;

use bevy::prelude::*;
use ratatui::{buffer::Cell, style::Modifier};

use super::{
  CanvasArgs, DrawnShape, LineCap, LineStyle, LineVariant, ShapeBuffer,
  thin_neighbor::{Neighbor, thin_neighbor_symbol},
};

pub struct LineArgs {
  pub from:  Vec3,
  pub to:    Vec3,
  pub style: LineStyle,
}

impl DrawnShape for LineArgs {
  fn draw(
    &self,
    buffer: &mut ShapeBuffer,
    args: &CanvasArgs,
    transform: &Transform,
  ) {
    let LineArgs { from, to, style } = self;

    let from = transform.transform_point(*from);
    let to = transform.transform_point(*to);

    let canvas_from = args.world_to_canvas_coords(from);
    let canvas_to = args.world_to_canvas_coords(to);

    let points = match style.variant {
      LineVariant::Thin => basic_8_connected(canvas_from, canvas_to),
    };

    for (i, (point, t)) in points.iter().enumerate() {
      // get the angle between the next point and the previous point
      let next_point_index = (i + 1).min(points.len() - 1);
      let next_point = points[next_point_index].0;
      let prev_point = points[i.saturating_sub(1)].0;
      let next_point_offset = next_point - point;
      let prev_point_offset = prev_point - point;

      let is_end = i == 0 || i == points.len() - 1;
      let mut cell = match style.cap {
        Some(LineCap::Plus) if is_end => Cell::new("+"),
        _ => Cell::new(thin_neighbor_symbol(
          Neighbor::find(prev_point_offset),
          Neighbor::find(next_point_offset),
        )),
      };
      cell.modifier = Modifier::BOLD;
      cell.fg = style.fg;
      if let Some(bg) = style.bg {
        cell.bg = bg;
      }

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

  let step_x = match p1.x.cmp(&p2.x) {
    Ordering::Less => 1,
    Ordering::Greater => -1,
    Ordering::Equal => 0,
  };
  let step_y = match p1.y.cmp(&p2.y) {
    Ordering::Less => 1,
    Ordering::Greater => -1,
    Ordering::Equal => 0,
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
