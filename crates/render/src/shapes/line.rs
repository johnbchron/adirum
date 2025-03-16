use std::cmp::Ordering;

use bevy::prelude::*;

use super::{
  CanvasArgs, DrawnShape, Material, MaterialDrawRequest,
  MaterialDrawRequestType, ProjectedPoint, ShapeBuffer,
  thin_neighbor::Neighbor,
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

    for (i, p) in points.iter().enumerate() {
      // get the previous and next neighbors, using self if at extent
      let next_point_index = (i + 1).min(points.len() - 1);
      let next_point = points[next_point_index].pos();
      let prev_point = points[i.saturating_sub(1)].pos();
      let next_point_offset = next_point - p.pos();
      let prev_point_offset = prev_point - p.pos();
      let prev_neighbor =
        Neighbor::find(prev_point_offset, args.character_aspect_ratio());
      let next_neighbor =
        Neighbor::find(next_point_offset, args.character_aspect_ratio());

      // are we on the end cap
      let is_end = i == 0 || i == points.len() - 1;

      // select the material for this cell
      let material = match style.cap_material.clone() {
        Some(cap_mat) if is_end => cap_mat,
        _ => style.material.clone(),
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
      let drawn_material = material.draw(request, p.depth());

      buffer.draw(drawn_material, *p);
    }
  }
}

pub fn basic_8_connected(
  mut p1: ProjectedPoint,
  p2: ProjectedPoint,
) -> Vec<ProjectedPoint> {
  let mut result = Vec::new();

  let delta = (p2.pos() - p1.pos()).abs();
  let mut current = p1.pos();
  let steps = delta.x.max(delta.y);

  let depth_step = (p2.depth() - p1.depth()) / steps as f32;

  result.push(ProjectedPoint::new(current, p1.depth()));

  let step_x = match p1.pos().x.cmp(&p2.pos().x) {
    Ordering::Less => 1,
    Ordering::Greater => -1,
    Ordering::Equal => 0,
  };
  let step_y = match p1.pos().y.cmp(&p2.pos().y) {
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

    p1.set_depth(p1.depth() + depth_step);
    result.push(ProjectedPoint::new(current, p1.depth()));
  }

  result
}
