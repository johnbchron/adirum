use bevy::prelude::*;

use crate::render::camera::MainCamera;

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
  mut query: Query<&mut Transform, With<MainCamera>>,
  time: Res<Time>,
) {
  let mut move_dir = Vec3::ZERO;

  if keyboard.pressed(KeyCode::KeyW) {
    move_dir -= Vec3::Z;
  }
  if keyboard.pressed(KeyCode::KeyS) {
    move_dir += Vec3::Z;
  }
  if keyboard.pressed(KeyCode::KeyA) {
    move_dir -= Vec3::X;
  }
  if keyboard.pressed(KeyCode::KeyD) {
    move_dir += Vec3::X;
  }
  if keyboard.pressed(KeyCode::Space) {
    move_dir += Vec3::Y;
  }
  if keyboard.pressed(KeyCode::ShiftLeft) {
    move_dir -= Vec3::Y;
  }

  for mut transform in query.iter_mut() {
    transform.translation += move_dir * time.delta_secs();
    *transform = transform.looking_at(Vec3::ZERO, Vec3::Y);
  }
}
