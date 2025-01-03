use bevy::prelude::*;

use crate::render::camera::{Camera, MainCamera};

#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (keyboard_input_app_exit, keyboard_input_move_camera),
    );
  }
}

fn keyboard_input_app_exit(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut exit: EventWriter<AppExit>,
) {
  if keyboard.pressed(KeyCode::KeyQ) {
    exit.send(AppExit::Success);
  }
}

fn keyboard_input_move_camera(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut query: Query<(&mut Transform, &mut Camera), With<MainCamera>>,
  time: Res<Time>,
) {
  let mut local_move_dir = Vec3::ZERO;
  let zoom_speed = 0.2;
  let mut zoom_dir = 0.0;

  if keyboard.pressed(KeyCode::KeyW) {
    local_move_dir += Vec3::Y;
  }
  if keyboard.pressed(KeyCode::KeyS) {
    local_move_dir -= Vec3::Y;
  }
  if keyboard.pressed(KeyCode::KeyA) {
    local_move_dir -= Vec3::X;
  }
  if keyboard.pressed(KeyCode::KeyD) {
    local_move_dir += Vec3::X;
  }

  if keyboard.pressed(KeyCode::ArrowUp) {
    zoom_dir += 1.0;
  }
  if keyboard.pressed(KeyCode::ArrowDown) {
    zoom_dir -= 1.0;
  }

  for (mut transform, mut camera) in query.iter_mut() {
    let move_dir = transform.compute_matrix().transform_vector3(local_move_dir);
    transform.translation += move_dir * time.delta_secs();

    let current_scale = camera.scale();
    camera.set_scale(
      current_scale * (1.0 + (-zoom_dir * zoom_speed * time.delta_secs())),
    );
  }
}
