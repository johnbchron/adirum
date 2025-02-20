use std::cmp::Ordering;

use bevy::prelude::*;

use super::{
  CanvasArgs, DrawnShape, Material, MaterialDrawRequest,
  MaterialDrawRequestType, ShapeBuffer, thin_neighbor::Neighbor,
};

pub struct LineArgs {
  pub from:  Vec3,
  pub to:    Vec3,
  pub style: LineStyle,
}

#[derive(Clone)]
pub struct LineStyle {
  pub material:     Material,
  pub cap_material: Option<Material>,
  pub variant:      LineVariant,
}

#[derive(Clone, Copy)]
pub enum LineVariant {
  Thin,
}

impl DrawnShape for LineArgs {
  fn draw(
    &self,
    buffer: &mut ShapeBuffer,
    args: &CanvasArgs,
    transform: &Transform,
  ) {
    let LineArgs { from, to, style } = self;

    let transformed_from = transform.transform_point(*from);
    let transformed_to = transform.transform_point(*to);

    let canvas_from = args.world_to_canvas_coords(transformed_from);
    let canvas_to = args.world_to_canvas_coords(transformed_to);

    let points = match style.variant {
      LineVariant::Thin => basic_8_connected(canvas_from, canvas_to),
    };

    for (i, (position, proj_depth)) in points.iter().enumerate() {
      // get the previous and next neighbors, using self if at extent
      let next_point_index = (i + 1).min(points.len() - 1);
      let next_point = points[next_point_index].0;
      let prev_point = points[i.saturating_sub(1)].0;
      let next_point_offset = next_point - position;
      let prev_point_offset = prev_point - position;
      let prev_neighbor =
        Neighbor::find(prev_point_offset, args.character_aspect_ratio());
      let next_neighbor =
        Neighbor::find(next_point_offset, args.character_aspect_ratio());

      // are we on the end cap
      let is_end = i == 0 || i == points.len() - 1;

      // select the material for this cell
      let material = match style.cap_material {
        Some(cap_mat) if is_end => cap_mat,
        _ => style.material,
      };

      // figure out what info we need
      let mat_request_type = material.draw_request_type();

      // fill in the info
      let request = match mat_request_type {
        MaterialDrawRequestType::None => MaterialDrawRequest::None,
        MaterialDrawRequestType::Neighbors => MaterialDrawRequest::Neighbors {
          prev: prev_neighbor,
          next: next_neighbor,
        },
      };

      // determine the character
      let drawn_material = material.draw(request, *proj_depth);

      buffer.draw(drawn_material, *position, *proj_depth);
    }
  }
}

pub fn basic_8_connected(
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
