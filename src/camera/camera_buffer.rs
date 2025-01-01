use std::ops::DerefMut;

use bevy::prelude::*;
use ratatui::{
  buffer::Cell,
  prelude::{Rect, *},
};

use super::{Camera, Material};
use crate::{
  colors::{BACKGROUND_COLOR_RATATUI, PUNCHY_TEXT_COLOR_RATATUI},
  ui::RenderedWidgetState,
};

#[derive(Resource)]
pub struct CameraBuffer {
  camera:        Camera,
  render_buffer: Vec<Material>,
  widget_state:  RenderedWidgetState,
}

impl CameraBuffer {
  pub fn new(camera: Camera) -> Self {
    Self {
      camera,
      render_buffer: vec![],
      widget_state: RenderedWidgetState::new(),
    }
  }

  pub fn widget_state_mut(&mut self) -> &mut RenderedWidgetState {
    &mut self.widget_state
  }

  fn render_area(&self) -> Rect { self.widget_state.last_area() }

  /// Clears the render buffer and resizes it based off the area in the widget
  /// state.
  fn prepare_for_render(&mut self) {
    self
      .render_buffer
      .resize(self.render_area().area() as usize, Material::Nothing);
  }

  /// Resizes the widget buffer and updates it with the contents of the render
  /// buffer.
  fn update_widget_buffer(&mut self) {
    let render_area = self.render_area();
    let buffer = self.widget_state.buffer_mut();

    buffer.resize(render_area);

    for y in 0..render_area.height {
      for x in 0..render_area.width {
        let index = (y * render_area.width + x) as usize;

        let cell = match self.render_buffer[index] {
          Material::Wall => {
            let mut cell = Cell::new("#");
            cell.set_bg(BACKGROUND_COLOR_RATATUI);
            cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
            cell
          }
          Material::WallCorner => {
            let mut cell = Cell::new("+");
            cell.set_bg(BACKGROUND_COLOR_RATATUI);
            cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
            cell
          }
          Material::Nothing => {
            let mut cell = Cell::new(" ");
            cell.set_bg(BACKGROUND_COLOR_RATATUI);
            // cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
            cell
          }
        };

        let output_index = y * render_area.width + x;
        buffer.content.deref_mut()[output_index as usize] = cell;
      }
    }
  }
}

impl Default for CameraBuffer {
  fn default() -> Self { Self::new(Camera::default()) }
}

pub fn prepare_for_render(
  camera: Res<Camera>,
  mut camera_buffer: ResMut<CameraBuffer>,
) {
  camera_buffer.camera = camera.clone();
  camera_buffer.prepare_for_render();
}

pub fn dummy_render(mut camera_buffer: ResMut<CameraBuffer>) {
  let render_area = camera_buffer.render_area();

  for y in 0..render_area.height {
    for x in 0..render_area.width {
      let index = (y * render_area.width + x) as usize;

      camera_buffer.render_buffer[index] = Material::Wall;
    }
  }
}

pub fn finalize_render(mut camera_buffer: ResMut<CameraBuffer>) {
  camera_buffer.update_widget_buffer();
}
