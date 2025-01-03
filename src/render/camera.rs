use bevy::prelude::*;

use super::render_buffer::RenderBufferSize;
pub use crate::render::render_buffer::RenderBuffer;

/// Standard orthographic camera
#[derive(Component, Clone)]
pub struct Camera {
  /// The aspect ratio of the terminal characters.
  ///
  /// This, combined with the render buffer's aspect ratio, determines the
  /// aspect ratio of the orthographic projection.
  character_aspect_ratio: f32,
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      // assume that the terminal characters are twice as tall as they are wide
      character_aspect_ratio: 0.5,
    }
  }
}

impl Camera {
  pub fn new(character_aspect_ratio: f32) -> Self {
    Self {
      character_aspect_ratio,
    }
  }

  /// Calculates an orthographic projection matrix for the camera.
  pub fn calculate_matrix(
    &self,
    camera_transform: &Transform,
    render_buffer_size: &RenderBufferSize,
  ) -> CameraMatrix {
    let aspect_ratio = render_buffer_size.0 as f32
      / render_buffer_size.1 as f32
      * self.character_aspect_ratio;
    let ortho_height = 1.0;
    let ortho_width = ortho_height * aspect_ratio;

    let proj = Mat4::orthographic_rh(
      -ortho_width,
      ortho_width,
      -ortho_height,
      ortho_height,
      0.0,
      1000.0,
    );
    let view = camera_transform.compute_matrix().inverse();

    CameraMatrix { proj, view }
  }
}

#[derive(Component, Clone, Debug, Default)]
pub struct CameraMatrix {
  pub proj: Mat4,
  pub view: Mat4,
}

#[derive(Component)]
pub struct MainCamera;

pub fn update_camera_matrices(
  mut query: Query<(Entity, &Camera, &Transform, Option<&mut CameraMatrix>)>,
  render_buffer_size: Res<RenderBufferSize>,
  mut commands: Commands,
) {
  for (entity, camera, camera_transform, existing_matrix) in query.iter_mut() {
    let matrix = camera.calculate_matrix(camera_transform, &render_buffer_size);

    if let Some(mut existing_matrix) = existing_matrix {
      *existing_matrix = matrix;
    } else {
      commands.entity(entity).insert(matrix);
    }
  }
}
