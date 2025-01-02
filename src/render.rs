pub mod camera;
pub mod render_buffer;

use ratatui::buffer::Cell;

use crate::colors::{BACKGROUND_COLOR_RATATUI, PUNCHY_TEXT_COLOR_RATATUI};

#[derive(Clone)]
pub enum Material {
  Wall,
  WallCorner,
  Nothing,
}

impl Material {
  pub fn to_cell(&self) -> Cell {
    match self {
      Material::Wall => {
        let mut cell = Cell::new("#");
        cell.set_bg(BACKGROUND_COLOR_RATATUI);
        cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
        cell
      }
      Material::WallCorner => {
        let mut cell = Cell::new("+");
        cell.set_bg(BACKGROUND_COLOR_RATATUI);
        cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
        cell
      }
      Material::Nothing => {
        let mut cell = Cell::new(" ");
        cell.set_bg(BACKGROUND_COLOR_RATATUI);
        cell.set_fg(PUNCHY_TEXT_COLOR_RATATUI);
        cell
      }
    }
  }
}
