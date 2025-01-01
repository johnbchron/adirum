use std::ops::DerefMut;

use bevy::prelude::*;
use ratatui::{
  buffer::Cell,
  prelude::{Rect, *},
};

use super::{Camera, Material};
use crate::ui::RenderedWidgetState;

#[derive(Resource)]
pub struct CameraBuffer {
  camera:        Camera,
  render_buffer: Vec<Material>,
  widget_state:  RenderedWidgetState,
}

impl CameraBuffer {
  pub fn new(camera: Camera, side_length: u16, material: Material) -> Self {
    Self {
      camera,
      render_buffer: vec![
        material;
        (side_length * side_length * side_length) as usize
      ],
      widget_state: RenderedWidgetState::new(Buffer::empty(Rect::new(
        0, 0, 0, 0,
      ))),
    }
  }

  pub fn widget_state_mut(&mut self) -> &mut RenderedWidgetState {
    &mut self.widget_state
  }

  fn side_length(&self) -> u16 {
    self
      .widget_state
      .last_area()
      .width
      .max(self.widget_state.last_area().height)
  }

  /// Clears the render buffer and resizes it based off the area in the widget
  /// state.
  fn prepare_for_render(&mut self) {
    let side_length = self.side_length();

    self.render_buffer.clear();

    self.render_buffer.resize(
      (side_length as u32 * side_length as u32 * side_length as u32) as usize,
      Material::Nothing,
    );
  }

  /// Resizes the widget buffer and updates it with the contents of the render
  /// buffer.
  fn update_widget_buffer(&mut self) {
    let side_length = self.side_length() as u32;
    let buffer = self.widget_state.buffer_mut();

    buffer.resize(Rect::new(
      0,
      0,
      side_length.try_into().unwrap(),
      side_length.try_into().unwrap(),
    ));

    for z in 0..side_length {
      for y in 0..side_length {
        for x in 0..side_length {
          let index =
            (z * side_length * side_length + y * side_length + x) as usize;

          let cell = match self.render_buffer[index] {
            Material::Wall => Cell::new("#"),
            Material::WallCorner => Cell::new("+"),
            Material::Nothing => Cell::new(" "),
          };

          let output_index = y * side_length + x;
          buffer.content.deref_mut()[output_index as usize] = cell;
        }
      }
    }
  }
}

impl Default for CameraBuffer {
  fn default() -> Self { Self::new(Camera::default(), 0, Material::Nothing) }
}

pub fn prepare_for_render(
  camera: Res<Camera>,
  mut camera_buffer: ResMut<CameraBuffer>,
) {
  camera_buffer.camera = camera.clone();
  camera_buffer.prepare_for_render();
}

pub fn dummy_render(mut camera_buffer: ResMut<CameraBuffer>) {
  let side_length = camera_buffer.side_length() as u32;

  for z in 0..side_length {
    for y in 0..side_length {
      for x in 0..side_length {
        let index =
          (z * side_length * side_length + y * side_length + x) as usize;

        camera_buffer.render_buffer[index] = Material::Wall;
      }
    }
  }
}

pub fn finalize_render(mut camera_buffer: ResMut<CameraBuffer>) {
  camera_buffer.update_widget_buffer();
}
