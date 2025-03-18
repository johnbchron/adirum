use bevy::prelude::*;
use ratatui::{
  style::Stylize,
  text::Text,
  widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::{
  Render,
  shapes::{CanvasArgs, RenderedShape},
};

#[derive(Debug, Component, Default)]
#[require(RenderedShape, Transform)]
pub struct DebugSign {
  infos: Vec<String>,
}

impl DebugSign {
  fn render(&self) -> Paragraph {
    Paragraph::new(Text::from_iter(
      self.infos.iter().flat_map(|s| Text::from(s.clone()).lines),
    ))
    .bg(colors::BACKGROUND_COLOR_RATATUI)
    .fg(colors::NORMAL_TEXT_COLOR_RATATUI)
    .wrap(Wrap { trim: false })
    .block(
      Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Entity Debug".fg(colors::TITLE_COLOR_RATATUI))
        .fg(colors::NORMAL_BORDER_COLOR_RATATUI),
    )
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
fn propagate_infos(world: &mut World) {
  let mut query = world.query_filtered::<Entity, With<DebugSign>>();
  let entities = query.iter(world).collect::<Vec<_>>();

  for entity in entities {
    let reflections = world
      .inspect_entity(entity)
      .filter_map(|ci| ci.type_id())
      .filter_map(|ti| world.get_reflect(entity, ti).ok())
      .map(|r| format!("{r:?}"))
      .collect::<Vec<_>>();

    let Some(mut ds) = world.get_mut::<DebugSign>(entity) else {
      continue;
    };

    for reflection in reflections {
      ds.infos.push(reflection);
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
      .add_systems(PostUpdate, propagate_infos)
      .add_systems(Render, render_signs);
  }
}
