use ratatui::prelude::*;
use render::render_buffer::RenderedWidgetState;

pub struct RenderedWidget;

impl StatefulWidget for RenderedWidget {
  type State = RenderedWidgetState;

  fn render(
    self,
    target_area: Rect,
    buf: &mut Buffer,
    state: &mut Self::State,
  ) {
    // make sure that the next render is synced to the current size
    *state.last_area_mut() = target_area;

    // center the render buffer in the target area
    let x_offset = ((target_area.width as i32
      - state.buffer().area.width as i32)
      / 2)
      .max(0) as u16;
    let y_offset =
      ((target_area.height as i32 - state.buffer().area.height as i32) / 2)
        .max(0) as u16;
    state.buffer_mut().area.x = target_area.x + x_offset;
    state.buffer_mut().area.y = target_area.y + y_offset;

    // resize the render buffer to the intersection of itself and the target
    let new_area = target_area.intersection(state.buffer().area);
    state.buffer_mut().resize(new_area);

    // copy the render buffer to the main buffer
    buf.merge(state.buffer());
  }
}
