use bevy::prelude::*;
use message::{MessageSender, MessageType};
use render::camera::{Camera, MainCamera};

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
  mut sender: MessageSender,
) {
  let move_speed = 1.0;
  let mut local_move_dir = Vec3::ZERO;
  let zoom_speed = 0.5;
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
    let current_scale = camera.scale();
    let scale_delta_coefficient =
      1.0 + (zoom_dir * zoom_speed * time.delta_secs());
    camera.set_scale(current_scale * scale_delta_coefficient);
    if scale_delta_coefficient != 1.0 {
      sender.send(MessageType::MutateCameraScale(scale_delta_coefficient));
    }

    let move_dir = transform.compute_matrix().transform_vector3(local_move_dir);
    let move_amount =
      move_dir * current_scale.recip() * move_speed * time.delta_secs();
    transform.translation += move_amount;
    if move_amount != Vec3::ZERO {
      sender.send(MessageType::MutateCameraMove(move_amount));
    }
  }
}
