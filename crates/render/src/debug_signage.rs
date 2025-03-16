use bevy::prelude::*;
use message::MessageSender;
use ratatui::{
  style::Stylize,
  text::Text,
  widgets::{Block, BorderType, Paragraph},
};

use crate::{
  Render,
  shapes::{CanvasArgs, RenderedShape},
};

#[derive(Debug, Component, Default)]
struct DebugSignRequired;

#[derive(Debug, Component, Default)]
#[require(RenderedShape, Transform)]
struct DebugSignChild {
  infos: Vec<DebugSignInfoItem>,
}

impl DebugSignChild {
  fn render(&self) -> Paragraph {
    let mut text = Vec::with_capacity(1 + self.infos.len());
    text.push(Text::from("  Debug Info:").bold());
    for info in &self.infos {
      text.push(info.render());
    }
    Paragraph::new(text.into_iter().flat_map(|t| t.lines).collect::<Vec<_>>())
      .bg(colors::BACKGROUND_COLOR_RATATUI)
      .fg(colors::NORMAL_TEXT_COLOR_RATATUI)
      .block(
        Block::bordered()
          .border_type(BorderType::Rounded)
          .title("Entity Debug".fg(colors::TITLE_COLOR_RATATUI))
          .fg(colors::NORMAL_BORDER_COLOR_RATATUI),
      )
  }
}

#[derive(Debug, Component)]
#[require(DebugSignRequired)]
pub struct DebugSignTransform;

#[derive(Debug)]
enum DebugSignInfoItem {
  Transform(Transform),
}

impl DebugSignInfoItem {
  fn render(&self) -> Text {
    match self {
      DebugSignInfoItem::Transform(transform) => Text::from(
        serde_json::to_string_pretty(transform)
          .expect("failed to serialize `Transform`"),
      ),
    }
  }
}

pub(crate) struct DebugSignPlugin;

fn clear_infos(mut query: Query<&mut DebugSignChild>) {
  for mut dsc in query.iter_mut() {
    dsc.infos.clear();
  }
}

/// For each entity with a `DebugSignRequired`, find all the children with a
/// `DebugSignChild`, and add the info from the parent to the `DebugSignChild`.
#[allow(clippy::type_complexity)]
fn propagate_infos(
  query: Query<
    (Option<&DebugSignTransform>, Option<&Transform>, &Children),
    With<DebugSignRequired>,
  >,
  mut child_query: Query<&mut DebugSignChild>,
) {
  for (transform_flag, parent_transform, children) in query.iter() {
    let mut child_dsc_iter = child_query.iter_many_mut(children);
    while let Some(mut child_dsc) = child_dsc_iter.fetch_next() {
      if let (Some(_), Some(parent_transform)) =
        (transform_flag, parent_transform)
      {
        child_dsc
          .infos
          .push(DebugSignInfoItem::Transform(*parent_transform));
      }
    }
  }
}

fn render_signs(
  canvas_args: CanvasArgs,
  mut query: Query<(&DebugSignChild, &Transform, &mut RenderedShape)>,
) {
  use crate::shapes::*;

  for (dsc, transform, mut buffer) in query.iter_mut() {
    let sign = SignArgs {
      content:    dsc.render(),
      max_width:  40,
      max_height: None,
      position:   Vec3::ZERO,
      anchor:     Vec2::new(1.0, -1.0),
      on_top:     true,
    };

    sign.draw(buffer.inner_mut(), &canvas_args, transform);
  }
}

fn spawn_children(
  mut commands: Commands,
  mut message_sender: MessageSender,
  query: Query<(Entity, Option<&Children>), With<DebugSignRequired>>,
  child_query: Query<Entity, With<DebugSignChild>>,
) {
  for (parent, parent_children) in query.iter() {
    // skip if there's a child with a DebugSignChild
    if let Some(parent_children) = parent_children {
      let mut child_iter = child_query.iter_many(parent_children);
      if child_iter.fetch_next().is_some() {
        continue;
      }
    }

    message_sender.send(message::MessageType::SpawnDebugSignChild { parent });
    commands
      .entity(parent)
      .with_child(DebugSignChild::default());
  }
}

/// For each entity with a `DebugSignChild`, despawn it if its parent doesn't
/// have `DebugSignRequired`.
fn despawn_children(
  mut commands: Commands,
  mut message_sender: MessageSender,
  query: Query<(Entity, &Parent), With<DebugSignChild>>,
  parent_query: Query<&DebugSignRequired>,
) {
  for (child, child_parent) in query.iter() {
    if parent_query.get(child_parent.get()).is_err() {
      message_sender.send(message::MessageType::DespawnDebugSignChild {
        parent: child_parent.get(),
        child,
      });
      commands
        .entity(child_parent.get())
        .remove_children(&[child]);
      commands.entity(child).despawn_recursive();
    }
  }
}

impl Plugin for DebugSignPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PreUpdate, (spawn_children, despawn_children, clear_infos))
      .add_systems(Update, propagate_infos)
      .add_systems(Render, render_signs);
  }
}
