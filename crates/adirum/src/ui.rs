mod diagnostic_bar_widget;
mod message_log_widget;
mod rendered_widget;
mod styles;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::{error::exit_on_error, terminal::RatatuiContext};
use diagnostic_bar_widget::DiagnosticBarWidget;
use ratatui::{
  layout::{Constraint, Layout},
  widgets::{Block, StatefulWidget, Widget},
};
use rendered_widget::RenderedWidget;

pub use self::rendered_widget::RenderedWidgetState;
use self::{message_log_widget::MessageLogWidget, styles::BASE_STYLE};
use crate::{
  message::{MessageLog, MessageLogWidgetAnimationSettings},
  render::render_buffer::RenderBuffer,
};

pub struct UiApp<'a> {
  camera_buffer: ResMut<'a, RenderBuffer>,
  diagnostic_store: Res<'a, DiagnosticsStore>,
  message_log: Res<'a, MessageLog>,
  message_log_anim_settings: Res<'a, MessageLogWidgetAnimationSettings>,
  time: Res<'a, Time>,
}

impl Widget for UiApp<'_> {
  fn render(
    mut self,
    area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
  ) {
    Block::new().style(BASE_STYLE).render(area, buf);

    let layout = Layout::vertical([
      Constraint::Length(1),
      Constraint::Min(0),
      Constraint::Length(10),
    ])
    .split(area);

    DiagnosticBarWidget::new(self.diagnostic_store).render(layout[0], buf);

    RenderedWidget.render(
      layout[1],
      buf,
      self.camera_buffer.widget_state_mut(),
    );

    MessageLogWidget::new(
      self.message_log,
      self.message_log_anim_settings,
      self.time,
    )
    .render(layout[2], buf);
  }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Last, draw_ui.pipe(exit_on_error));
  }
}

pub fn draw_ui(
  mut context: ResMut<RatatuiContext>,
  camera_buffer: ResMut<RenderBuffer>,
  diagnostic_store: Res<DiagnosticsStore>,
  message_log: Res<MessageLog>,
  message_log_anim_settings: Res<MessageLogWidgetAnimationSettings>,
  time: Res<Time>,
) -> color_eyre::Result<()> {
  context.draw(|frame| -> _ {
    frame.render_widget(
      UiApp {
        camera_buffer,
        diagnostic_store,
        message_log,
        message_log_anim_settings,
        time,
      },
      frame.area(),
    )
  })?;

  Ok(())
}
