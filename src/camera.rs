mod camera_buffer;

use bevy::prelude::*;

pub use self::camera_buffer::CameraBuffer;
use self::camera_buffer::{dummy_render, finalize_render, prepare_for_render};

#[derive(Clone)]
pub enum Material {
  Wall,
  WallCorner,
  Nothing,
}

#[derive(Resource, Clone)]
pub struct Camera {
  viewing_volume: Transform,
}

impl Camera {
  fn new(viewing_volume: Transform) -> Self { Self { viewing_volume } }
  fn cabinet(center: Vec3, size: Vec3, from: Vec3) -> Self {
    Self::new(
      Transform::from_translation(center)
        .with_scale(size)
        .looking_at(from, Vec3::Y),
    )
  }
}

impl Default for Camera {
  fn default() -> Self {
    Self::cabinet(Vec3::ZERO, Vec3::ONE, Vec3::new(1.0, 1.0, -2.0))
  }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<Camera>()
      .init_resource::<CameraBuffer>()
      .add_systems(
        PostUpdate,
        (prepare_for_render, dummy_render, finalize_render).chain(),
      );
  }
}
