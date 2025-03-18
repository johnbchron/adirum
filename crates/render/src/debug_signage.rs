use bevy::prelude::*;
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
#[require(RenderedShape, Transform)]
struct DebugSign {
  infos: Vec<DebugSignInfoItem>,
}

impl DebugSign {
  fn render(&self) -> Paragraph {
    let mut text = Vec::with_capacity(1 + self.infos.len());
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
#[require(DebugSign)]
pub struct DebugSignTransform;

#[derive(Debug)]
enum DebugSignInfoItem {
  Transform(Transform),
}

impl DebugSignInfoItem {
  fn render(&self) -> Text {
    match self {
      DebugSignInfoItem::Transform(transform) => {
        Text::from(format!("{transform:#?}"))
      }
    }
  }
}

pub(crate) struct DebugSignPlugin;

fn clear_infos(mut query: Query<&mut DebugSign>) {
  for mut dsc in query.iter_mut() {
    dsc.infos.clear();
  }
}

/// For each entity with a `DebugSignRequired`, find all the children with a
/// `DebugSignChild`, and add the info from the parent to the `DebugSignChild`.
#[allow(clippy::type_complexity)]
fn propagate_infos(
  mut query: Query<(
    &mut DebugSign,
    Option<&DebugSignTransform>,
    Option<&Transform>,
  )>,
) {
  for (mut ds, transform_flag, transform) in query.iter_mut() {
    if let (Some(_), Some(transform)) = (transform_flag, transform) {
      ds.infos.push(DebugSignInfoItem::Transform(*transform));
    }
  }
}

fn render_signs(
  canvas_args: CanvasArgs,
  mut query: Query<(&DebugSign, &Transform, &mut RenderedShape)>,
) {
  use crate::shapes::*;

  for (ds, transform, mut buffer) in query.iter_mut() {
    let sign = SignArgs {
      content:    ds.render(),
      min_width:  Some(32),
      max_width:  40,
      max_height: None,
      position:   Vec3::ZERO,
      anchor:     Vec2::new(1.0, -1.0),
      on_top:     true,
    };

    sign.draw(buffer.inner_mut(), &canvas_args, transform);
  }
}

impl Plugin for DebugSignPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PreUpdate, clear_infos)
      .add_systems(Update, propagate_infos)
      .add_systems(Render, render_signs);
  }
}
