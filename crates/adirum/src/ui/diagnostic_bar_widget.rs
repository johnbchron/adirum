use bevy::{
  diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
  prelude::*,
};
use ratatui::{
  layout::Flex,
  prelude::{Rect, *},
  widgets::Block,
};
use render::diagnostics::{
  DRAWN_CELL_COUNT_DIAG_PATH, SHAPE_BUFFER_COUNT_DIAG_PATH,
};

use super::styles::{DEFAULT_STYLE, DIM_STYLE, PUNCHY_STYLE};

pub struct DiagnosticBarWidget<'a> {
  diagnostic_store: Res<'a, DiagnosticsStore>,
}

impl<'a> DiagnosticBarWidget<'a> {
  pub fn new(diagnostic_store: Res<'a, DiagnosticsStore>) -> Self {
    Self { diagnostic_store }
  }
}

impl Widget for DiagnosticBarWidget<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let params = &[
      ("SHAPE_BUFFER_COUNT", SHAPE_BUFFER_COUNT_DIAG_PATH),
      ("DRAWN_CELL_COUNT", DRAWN_CELL_COUNT_DIAG_PATH),
      ("FRAME_TIME", FrameTimeDiagnosticsPlugin::FRAME_TIME),
      ("FPS", FrameTimeDiagnosticsPlugin::FPS),
    ];

    let lines = params
      .iter()
      .filter_map(|(label, path)| {
        self
          .diagnostic_store
          .get_measurement(path)
          .map(|v| (label, v))
      })
      .map(|(label, value)| {
        Line::from_iter([
          Span::styled(format!("{label}: "), DIM_STYLE),
          Span::styled(format!("{:.03}", value.value), PUNCHY_STYLE),
        ])
      })
      .collect::<Vec<_>>();

    let layout = Layout::horizontal(
      lines.iter().map(|l| Constraint::Length(l.width() as _)),
    )
    .spacing(2)
    .flex(Flex::End)
    .split(area);

    Block::new().style(DEFAULT_STYLE).render(area, buf);

    lines.into_iter().enumerate().for_each(|(i, l)| {
      l.render(layout[i], buf);
    });
  }
}
