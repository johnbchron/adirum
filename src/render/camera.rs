use bevy::prelude::*;

use super::{MAX_PROJECTED_DEPTH, render_buffer::RenderBufferSize};
pub use crate::render::render_buffer::RenderBuffer;

/// Standard orthographic camera
#[derive(Component, Clone)]
pub struct Camera {
  /// The aspect ratio of the terminal characters.
  ///
  /// This, combined with the render buffer's aspect ratio, determines the
  /// aspect ratio of the orthographic projection.
  character_aspect_ratio: f32,
  /// The scale of the camera.
  scale:                  f32,
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      // charachter height in `em` is 1.2, and width is 0.5
      character_aspect_ratio: 5.0 / 12.0,
      scale:                  1.0,
    }
  }
}

impl Camera {
  pub fn new(character_aspect_ratio: f32, scale: f32) -> Self {
    Self {
      character_aspect_ratio,
      scale,
    }
  }

  pub fn scale(&self) -> f32 { self.scale }
  pub fn set_scale(&mut self, scale: f32) { self.scale = scale; }

  /// Calculates an orthogonal projection matrix for the camera.
  pub fn calculate_matrix(
    &self,
    camera_transform: &Transform,
    render_buffer_size: &RenderBufferSize,
  ) -> CameraMatrix {
    let aspect_ratio = render_buffer_size.0 as f32
      / render_buffer_size.1 as f32
      * self.character_aspect_ratio;
    let ortho_height = self.scale;
    let ortho_width = ortho_height * aspect_ratio;

    let mut proj = Mat4::orthographic_rh(
      -ortho_width,
      ortho_width,
      -ortho_height,
      ortho_height,
      0.0,
      MAX_PROJECTED_DEPTH,
    );

    let shear_angle = (-self.character_aspect_ratio.recip() / 2.0).atan();
    let foreshortening = -1.0 / 3.0;
    let cabinet = Mat4::from_cols(
      Vec4::new(1.0, 0.0, 0.0, 0.0),
      Vec4::new(0.0, 1.0, 0.0, 0.0),
      Vec4::new(
        -shear_angle.cos() * foreshortening,
        -shear_angle.sin() * foreshortening,
        1.0,
        0.0,
      ),
      Vec4::new(0.0, 0.0, 0.0, 1.0),
    );
    proj *= cabinet;

    let view = camera_transform.compute_matrix().inverse();

    CameraMatrix { proj, view }
  }
}

#[derive(Component, Clone, Debug, Default)]
pub struct CameraMatrix {
  proj: Mat4,
  view: Mat4,
}

impl CameraMatrix {
  pub fn world_to_view(&self, point: Vec3) -> Vec3 {
    self.view.transform_point3(point)
  }
  pub fn view_to_ndc(&self, point: Vec3) -> Vec3 {
    self.proj.transform_point3(point)
  }
  pub fn world_to_ndc(&self, point: Vec3) -> Vec3 {
    self.view_to_ndc(self.world_to_view(point))
  }
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
