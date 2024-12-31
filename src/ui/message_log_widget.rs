use bevy::prelude::*;
use ratatui::{
  prelude::{Rect, *},
  style::Color,
  widgets::{Block, BorderType, Paragraph},
};

use super::styles::{BORDER_STYLE, DEFAULT_STYLE, TITLE_STYLE};
use crate::message::{MessageLog, MessageLogWidgetAnimationSettings};

fn blend_u8_value(from: u8, to: u8, t: f32) -> u8 {
  (from as f32 + (to as f32 - from as f32) * t).round() as u8
}

fn blend_colors(from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
  let t = t.clamp(0.0, 1.0);
  (
    blend_u8_value(from.0, to.0, t),
    blend_u8_value(from.1, to.1, t),
    blend_u8_value(from.2, to.2, t),
  )
}

pub struct MessageLogWidget<'a> {
  message_log:   Res<'a, MessageLog>,
  anim_settings: Res<'a, MessageLogWidgetAnimationSettings>,
  time:          Res<'a, Time>,
}

impl<'a> MessageLogWidget<'a> {
  pub fn new(
    message_log: Res<'a, MessageLog>,
    anim_settings: Res<'a, MessageLogWidgetAnimationSettings>,
    time: Res<'a, Time>,
  ) -> Self {
    Self {
      message_log,
      anim_settings,
      time,
    }
  }
}

impl Widget for MessageLogWidget<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let lines = self
      .message_log
      .messages
      .iter()
      .rev()
      .map(|message| {
        let message_age = self.time.elapsed() - message.timestamp;
        let anim_t = message_age
          .div_duration_f32(self.anim_settings.opacity_anim_duration);

        let original_fg_color = match DEFAULT_STYLE.fg {
          Some(Color::Rgb(r, g, b)) => (r, g, b),
          _ => panic!("failed to extract FG RGB color from DEFAULT_STYLE"),
        };
        let bg_color = match DEFAULT_STYLE.bg {
          Some(Color::Rgb(r, g, b)) => (r, g, b),
          _ => panic!("failed to extract BG RGB color from DEFAULT_STYLE"),
        };
        let fg_color = blend_colors(bg_color, original_fg_color, anim_t);
        let style =
          DEFAULT_STYLE.fg(Color::Rgb(fg_color.0, fg_color.1, fg_color.2));

        Line::default().spans(vec![
          Span::styled(
            format!("{:.3}", message.timestamp.as_secs_f32()),
            style,
          )
          .bold(),
          Span::styled(format!(": {}", message.message), style),
        ])
      })
      .collect::<Vec<_>>();

    Paragraph::new(Text::from(lines))
      .block(
        Block::bordered()
          .border_style(BORDER_STYLE)
          .border_type(BorderType::Rounded)
          .title_style(TITLE_STYLE)
          .style(DEFAULT_STYLE)
          .title("Messages"),
      )
      .render(area, buf);
  }
}
