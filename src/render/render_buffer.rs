use std::ops::DerefMut;

use bevy::prelude::*;
use ratatui::{buffer::Cell, prelude::Rect};

use crate::{
  colors::{BACKGROUND_COLOR_RATATUI, PUNCHY_TEXT_COLOR_RATATUI},
  render::{Material, camera::Camera},
  ui::RenderedWidgetState,
};

#[derive(Resource)]
pub struct RenderBuffer {
  camera:        Camera,
  render_buffer: Vec<Material>,
  widget_state:  RenderedWidgetState,
}

impl RenderBuffer {
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

    for (index, material) in self.render_buffer.iter().enumerate() {
      buffer.content[index] = material.to_cell();
    }
  }
}

impl Default for RenderBuffer {
  fn default() -> Self { Self::new(Camera::default()) }
}

pub fn prepare_for_render(
  camera: Res<Camera>,
  mut camera_buffer: ResMut<RenderBuffer>,
) {
  camera_buffer.camera = camera.clone();
  camera_buffer.prepare_for_render();
}

pub fn dummy_render(mut camera_buffer: ResMut<RenderBuffer>) {
  let Rect { width, height, .. } = camera_buffer.render_area();

  for y in (height / 4)..(height * 3 / 4) {
    for x in (width / 4)..(width * 3 / 4) {
      let index = (y * width + x) as usize;

      camera_buffer.render_buffer[index] = Material::Wall;
    }
  }
}

pub fn finalize_render(mut camera_buffer: ResMut<RenderBuffer>) {
  camera_buffer.update_widget_buffer();
}
