use ratatui::prelude::*;

pub struct RenderedWidget;

pub struct RenderedWidgetState {
  last_area: Rect,
  buffer:    Buffer,
}

impl RenderedWidgetState {
  pub fn new() -> Self {
    Self {
      last_area: Rect::new(0, 0, 0, 0),
      buffer:    Buffer::empty(Rect::new(0, 0, 0, 0)),
    }
  }

  pub fn last_area(&self) -> Rect { self.last_area }
  pub fn buffer_mut(&mut self) -> &mut Buffer { &mut self.buffer }
}

impl StatefulWidget for RenderedWidget {
  type State = RenderedWidgetState;

  fn render(
    self,
    mut target_area: Rect,
    buf: &mut Buffer,
    state: &mut Self::State,
  ) {
    // make sure that the next render is synced to the current size
    state.last_area = target_area;

    // keep the original position of the target area and zero it out
    let target_area_original_x = target_area.x;
    let target_area_original_y = target_area.y;
    target_area.x = 0;
    target_area.y = 0;
    // also zero out the position of the buffer area
    state.buffer.area.x = 0;
    state.buffer.area.y = 0;

    // get the intersection of the target area and the buffer area
    let intersection = target_area.intersection(state.buffer.area);

    // resize the render buffer to the intersection
    state.buffer.resize(intersection);

    // add back the original position to the target area
    target_area.x = target_area_original_x;
    target_area.y = target_area_original_y;

    // center the render buffer in the target area
    let x_offset = (target_area.width - state.buffer.area.width) / 2;
    let y_offset = (target_area.height - state.buffer.area.height) / 2;
    state.buffer.area.x = target_area.x + x_offset;
    state.buffer.area.y = target_area.y + y_offset;

    // render the buffer to the main buffer
    buf.merge(&state.buffer);
  }
}
