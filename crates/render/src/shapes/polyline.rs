use bevy::prelude::*;

use super::{
  DrawnShape, MaterialDrawRequest, MaterialDrawRequestType, PolylineLoopStyle,
  PolylineStyle, basic_8_connected, thin_neighbor::Neighbor,
};

pub struct PolylineArgs {
  pub points: Vec<Vec3>,
  pub style:  PolylineStyle,
}

impl DrawnShape for PolylineArgs {
  fn draw(
    &self,
    buffer: &mut super::ShapeBuffer,
    args: &super::CanvasArgs,
    transform: &Transform,
  ) {
    let PolylineArgs {
      points: world_endpoints,
      style,
    } = self;

    if world_endpoints.len() < 2 {
      return;
    }

    let transformed_endpoints = world_endpoints
      .iter()
      .map(|p| transform.transform_point(*p));

    let canvas_endpoints = transformed_endpoints
      .map(|p| args.world_to_canvas_coords(p))
      .collect::<Vec<_>>();

    let endpoint_pairs =
      canvas_endpoints.iter().zip(canvas_endpoints.iter().skip(1));

    // tack on the last point pair if the polyline is closed
    let endpoint_pairs: Box<dyn Iterator<Item = _>> =
      if matches!(style.loop_style, PolylineLoopStyle::Closed { .. }) {
        Box::new(endpoint_pairs.chain(std::iter::once((
          canvas_endpoints.last().unwrap(),
          canvas_endpoints.first().unwrap(),
        ))))
      } else {
        Box::new(endpoint_pairs)
      };

    let mut point_sets = endpoint_pairs
      .map(|(from, to)| basic_8_connected(*from, *to))
      .collect::<Vec<_>>();

    // if a set contains no points, drop it
    point_sets.retain(|points| !points.is_empty());

    // check at the end of each set to see if the next set has the same point.
    // if so, keep the point with the lower depth (closer)
    for i in 0..point_sets.len() {
      let next_set = (i + 1) % point_sets.len();
      let at_end_of_set = point_sets[i].last();
      let at_start_of_next_set = point_sets[next_set].first();

      match (at_end_of_set, at_start_of_next_set) {
        (Some((end_point, end_depth)), Some((start_point, start_depth)))
          if end_point == start_point =>
        {
          if end_depth < start_depth {
            point_sets[i].pop();
          } else {
            point_sets[next_set].remove(0);
          }
        }
        _ => {}
      }
    }

    // the list of points, with a boolean that's true at the end of each set
    let points = point_sets
      .into_iter()
      .flat_map(|set| {
        let len = set.len();
        set
          .into_iter()
          .enumerate()
          .map(move |(i, (point, depth))| (point, depth, i == len - 1))
      })
      .collect::<Vec<_>>();

    const NEIGHBOR_STEP: usize = 1;

    for (i, (point, depth, is_cap)) in points.iter().enumerate() {
      let (next_point_index, prev_point_index) = match style.loop_style {
        PolylineLoopStyle::Open { .. } => {
          let next_point_index = (i + NEIGHBOR_STEP).min(points.len() - 1);
          let prev_point_index = i.saturating_sub(NEIGHBOR_STEP);
          (next_point_index, prev_point_index)
        }
        PolylineLoopStyle::Closed { .. } => {
          let next_point_index = (i + NEIGHBOR_STEP) % points.len();
          let prev_point_index =
            (i + points.len() - NEIGHBOR_STEP) % points.len();
          (next_point_index, prev_point_index)
        }
      };

      let next_point = points[next_point_index].0;
      let prev_point = points[prev_point_index].0;
      let next_point_offset = next_point - point;
      let prev_point_offset = prev_point - point;
      let prev_neighbor =
        Neighbor::find(prev_point_offset, args.character_aspect_ratio());
      let next_neighbor =
        Neighbor::find(next_point_offset, args.character_aspect_ratio());

      // select the material for this cell
      let material = match style.loop_style {
        PolylineLoopStyle::Open {
          point_cap_material,
          end_cap_material,
        } => {
          if *is_cap {
            end_cap_material.unwrap_or(style.material)
          } else {
            point_cap_material.unwrap_or(style.material)
          }
        }
        PolylineLoopStyle::Closed { point_cap_material } => {
          if *is_cap {
            point_cap_material.unwrap_or(style.material)
          } else {
            style.material
          }
        }
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
      let drawn_material = material.draw(request, *depth);

      buffer.draw(drawn_material, *point, *depth);
    }
  }
}
