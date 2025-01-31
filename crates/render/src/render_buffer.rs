use bevy::{math::vec2, prelude::*};
use ratatui::{buffer::Buffer, prelude::Rect};

use super::DEFAULT_CELL;

#[derive(Default)]
pub struct RenderedWidgetState {
  last_area: Rect,
  buffer:    Buffer,
}

impl RenderedWidgetState {
  pub fn last_area(&self) -> Rect { self.last_area }
  pub fn last_area_mut(&mut self) -> &mut Rect { &mut self.last_area }
  pub fn buffer(&self) -> &Buffer { &self.buffer }
  pub fn buffer_mut(&mut self) -> &mut Buffer { &mut self.buffer }
}

#[derive(Resource, Default, Clone)]
pub struct RenderBufferSize(UVec2);

const Y_FLIP: Vec2 = vec2(1.0, -1.0);

impl RenderBufferSize {
  pub fn ndc_to_canvas_coords(&self, point: Vec2) -> IVec2 {
    // map from [-1, 1] to [0, self.0], flipping y (y is down in canvas)
    ((point * Y_FLIP + 1.0) / 2.0 * self.0.as_vec2()).as_ivec2()
  }

  #[allow(dead_code)]
  pub fn canvas_to_ndc_coords(&self, point: IVec2) -> Vec2 {
    // map from [0, self.0] to [-1, 1], flipping y back
    (point.as_vec2() / self.0.as_vec2())
      .mul_add(Vec2::splat(2.0), Vec2::splat(-1.0))
      * Y_FLIP
  }

  pub fn aspect_ratio(&self) -> f32 { self.0.x as f32 / self.0.y as f32 }
}

#[derive(Resource, Default)]
pub struct RenderBuffer {
  widget_state: RenderedWidgetState,
}

impl RenderBuffer {
  pub fn new() -> Self {
    Self {
      widget_state: RenderedWidgetState::default(),
    }
  }

  pub fn widget_state_mut(&mut self) -> &mut RenderedWidgetState {
    &mut self.widget_state
  }

  pub fn render_area(&self) -> Rect { self.widget_state.last_area() }

  /// Clears the render buffer and resizes it based off the area in the widget
  /// state.
  fn update_render_buffer_size(&mut self) {
    let area = self.render_area();

    self.widget_state.buffer_mut().resize(area);
    self.widget_state.buffer_mut().content.fill(DEFAULT_CELL);
  }
}

pub fn prepare_for_frame(
  mut render_buffer_size: ResMut<RenderBufferSize>,
  mut render_buffer: ResMut<RenderBuffer>,
) {
  // propagate render area to `RenderBufferSize`
  let area = render_buffer.render_area();
  render_buffer_size.0.x = area.width as _;
  render_buffer_size.0.y = area.height as _;

  // resize the render buffer to what the widget used last
  render_buffer.update_render_buffer_size();
}
