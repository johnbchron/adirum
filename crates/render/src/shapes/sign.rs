use bevy::prelude::*;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, WidgetRef},
};

use super::{DrawnShape, Material, MaterialDrawRequest, ProjectedPoint};

pub struct SignArgs<'a> {
  /// The (`ratatui`) content of the sign.
  pub content:    Paragraph<'a>,
  /// The maximum width of the sign.
  pub max_width:  u16,
  /// The maximum height of the sign. This is optional.
  pub max_height: Option<u16>,
  /// The world-space position of the sign's anchor.
  pub position:   Vec3,
  /// The `[-1.0, 1.0]` relative position of the anchor within the sign.
  pub anchor:     Vec2,
  /// Whether to draw the sign on top of everything else.
  pub on_top:     bool,
}

impl DrawnShape for SignArgs<'_> {
  fn draw(
    &self,
    buffer: &mut super::ShapeBuffer,
    args: &super::CanvasArgs,
    transform: &Transform,
  ) {
    // determine actual content height
    let max_content_height = self.content.line_count(self.max_width) as u16;
    let content_height = self
      .max_height
      .map(|mh| mh.min(max_content_height))
      .unwrap_or(max_content_height);
    let content_width = (self.content.line_width() as u16).min(self.max_width);

    let content_size = Rect::new(0, 0, content_width, content_height);
    let content_half_extents =
      Vec2::new(content_width as f32 / 2.0, content_height as f32 / 2.0);
    let mut intermediate_buffer = Buffer::empty(content_size);

    self
      .content
      .render_ref(content_size, &mut intermediate_buffer);

    let world_space_anchor = transform.transform_point(self.position);
    let projected_anchor = args.world_to_canvas_coords(world_space_anchor);
    // the projected anchor point, plus the anchor position in canvas directions
    let content_center = projected_anchor.pos()
      + (content_half_extents * Vec2::new(-self.anchor.x, self.anchor.y))
        .round()
        .as_ivec2();
    // the point the paragraph is drawn from is the top left
    let content_origin = content_center - content_half_extents.as_ivec2();

    for (i, cell) in intermediate_buffer.content().iter().enumerate() {
      let buffer_pos = intermediate_buffer.pos_of(i);
      let buffer_pos = IVec2::new(buffer_pos.0 as _, buffer_pos.1 as _);
      let canvas_pos = content_origin + buffer_pos;
      let depth = match self.on_top {
        true => 0.0,
        false => projected_anchor.depth(),
      };

      let material = Material::Text {
        text:     cell.symbol().into(),
        fg_color: cell.fg,
        bg_color: cell.bg,
      };
      let material_draw_request_type = material.draw_request_type();
      let material_draw_request = match material_draw_request_type {
        super::MaterialDrawRequestType::None => MaterialDrawRequest::None,
        super::MaterialDrawRequestType::Neighbors => {
          panic!("Text cannot have line neighbors.");
        }
      };

      let drawn_material = material.draw(material_draw_request, depth);
      buffer.draw(drawn_material, ProjectedPoint::new(canvas_pos, depth));
    }
  }
}
