use ratatui::style::{Modifier, Style};

use crate::{
  BACKGROUND_COLOR_RATATUI, BASE_COLOR_RATATUI, DIM_TEXT_COLOR_RATATUI,
  NORMAL_BORDER_COLOR_RATATUI, NORMAL_TEXT_COLOR_RATATUI,
  PUNCHY_TEXT_COLOR_RATATUI, TITLE_COLOR_RATATUI,
};

pub const BASE_STYLE: Style = Style {
  fg:              Some(NORMAL_TEXT_COLOR_RATATUI),
  bg:              Some(BASE_COLOR_RATATUI),
  underline_color: None,
  add_modifier:    Modifier::empty(),
  sub_modifier:    Modifier::empty(),
};

pub const DEFAULT_STYLE: Style = Style {
  bg: Some(BACKGROUND_COLOR_RATATUI),
  ..BASE_STYLE
};

pub const PUNCHY_STYLE: Style = Style {
  fg: Some(PUNCHY_TEXT_COLOR_RATATUI),
  add_modifier: Modifier::BOLD,
  ..DEFAULT_STYLE
};

pub const DIM_STYLE: Style = Style {
  fg: Some(DIM_TEXT_COLOR_RATATUI),
  ..DEFAULT_STYLE
};

pub const BORDER_STYLE: Style = Style {
  fg: Some(NORMAL_BORDER_COLOR_RATATUI),
  ..DEFAULT_STYLE
};

pub const TITLE_STYLE: Style = Style {
  fg: Some(TITLE_COLOR_RATATUI),
  ..DEFAULT_STYLE
};
