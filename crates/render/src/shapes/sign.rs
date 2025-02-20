use bevy::prelude::*;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, WidgetRef},
};

use super::{DrawnShape, Material, MaterialDrawRequest};

pub struct SignArgs<'a> {
  pub content:    Paragraph<'a>,
  pub max_width:  u16,
  pub max_height: Option<u16>,
  pub position:   Vec3,
  pub anchor:     Vec2,
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

    let projected_anchor =
      args.world_to_canvas_coords(transform.transform_point(self.position));
    let content_center = projected_anchor.0
      + (content_half_extents * self.anchor).round().as_ivec2();
    let content_origin = content_center - content_half_extents.as_ivec2();

    for (i, cell) in intermediate_buffer.content().iter().enumerate() {
      let buffer_pos = intermediate_buffer.pos_of(i);
      let buffer_pos = IVec2::new(buffer_pos.0 as _, buffer_pos.1 as _);
      let canvas_pos = content_origin + buffer_pos;
      let depth = match self.on_top {
        true => 0.0,
        false => projected_anchor.1,
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
      buffer.draw(drawn_material, canvas_pos, depth);
    }
  }
}
