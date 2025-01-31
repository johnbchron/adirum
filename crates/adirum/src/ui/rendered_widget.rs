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
    target_area: Rect,
    buf: &mut Buffer,
    state: &mut Self::State,
  ) {
    // make sure that the next render is synced to the current size
    state.last_area = target_area;

    // center the render buffer in the target area
    let x_offset = ((target_area.width as i32 - state.buffer.area.width as i32)
      / 2)
      .max(0) as u16;
    let y_offset = ((target_area.height as i32
      - state.buffer.area.height as i32)
      / 2)
      .max(0) as u16;
    state.buffer.area.x = target_area.x + x_offset;
    state.buffer.area.y = target_area.y + y_offset;

    // resize the render buffer to the intersection of itself and the target
    state
      .buffer
      .resize(target_area.intersection(state.buffer.area));

    // copy the render buffer to the main buffer
    buf.merge(&state.buffer);
  }
}
