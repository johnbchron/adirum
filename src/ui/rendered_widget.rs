use ratatui::prelude::*;

pub struct RenderedWidget;

pub struct RenderedWidgetState {
  last_area: Rect,
  buffer:    Buffer,
}

impl RenderedWidgetState {
  pub fn new(buffer: Buffer) -> Self {
    Self {
      last_area: Rect::new(0, 0, 0, 0),
      buffer,
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

    // get the union of the target area and the buffer area
    let union = target_area.union(state.buffer.area);

    // center both the target area and the buffer area
    // in the union
    target_area.x = union.x + (union.width - target_area.width) / 2;
    target_area.y = union.y + (union.height - target_area.height) / 2;
    state.buffer.area.x = union.x + (union.width - state.buffer.area.width) / 2;
    state.buffer.area.y =
      union.y + (union.height - state.buffer.area.height) / 2;

    // create a temporary buffer to render the widget
    let mut temp_buf = Buffer::empty(target_area);

    // render the state's buffer to the temporary buffer
    temp_buf.merge(&state.buffer);

    // resize the temporary buffer to the target area
    temp_buf.resize(target_area);

    // move the temporary buffer to the original position
    temp_buf.area.x = target_area_original_x;
    temp_buf.area.y = target_area_original_y;

    // merge the temporary buffer to the main buffer
    buf.merge(&temp_buf);
  }
}
