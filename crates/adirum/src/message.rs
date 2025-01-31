use std::{fmt, time::Duration};

use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(Event, Clone)]
pub struct Message {
  pub message:   MessageType,
  pub timestamp: Duration,
}

#[derive(Clone)]
pub enum MessageType {
  Custom(String),
  MutateCameraScale(f32),
  MutateCameraMove(Vec3),
}

impl fmt::Display for MessageType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MessageType::Custom(message) => write!(f, "{}", message),
      MessageType::MutateCameraScale(zoom) => {
        write!(f, "scaling camera: {}x", zoom)
      }
      MessageType::MutateCameraMove(vec) => write!(f, "moving camera: {}", vec),
    }
  }
}

#[derive(Resource)]
pub struct MessageLog {
  pub messages: Vec<Message>,
}

impl Default for MessageLog {
  fn default() -> Self {
    MessageLog {
      messages: Vec::with_capacity(1000),
    }
  }
}

#[derive(Resource)]
pub struct MessageLogWidgetAnimationSettings {
  pub opacity_anim_duration: Duration,
}

impl Default for MessageLogWidgetAnimationSettings {
  fn default() -> Self {
    MessageLogWidgetAnimationSettings {
      opacity_anim_duration: Duration::from_millis(40),
    }
  }
}

#[derive(SystemParam)]
pub struct MessageSender<'w> {
  message_writer: EventWriter<'w, Message>,
  time:           Res<'w, Time>,
}

impl MessageSender<'_> {
  pub fn send(&mut self, message: MessageType) {
    self.message_writer.send(Message {
      message,
      timestamp: self.time.elapsed(),
    });
  }
}

fn write_messages_to_message_log(
  mut message_log: ResMut<MessageLog>,
  mut messages: EventReader<Message>,
) {
  for message in messages.read() {
    message_log.messages.push(message.clone());
  }
}

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<MessageLog>()
      .init_resource::<MessageLogWidgetAnimationSettings>()
      .add_event::<Message>()
      .add_systems(PostUpdate, write_messages_to_message_log);
  }
}
