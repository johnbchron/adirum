use colors::{
  BASE_COLOR_RATATUI, DIM_TEXT_COLOR_RATATUI, LINEART_COLOR_RATATUI,
};
use ratatui::{buffer::Cell, style::Color};

use super::thin_neighbor::{Neighbor, thin_neighbor_symbol};

fn blend_u8_value(from: u8, to: u8, t: f32) -> u8 {
  (from as f32 + (to as f32 - from as f32) * t).round() as u8
}

fn blend_color(from: Color, to: Color, t: f32) -> Color {
  let t = t.clamp(0.0, 1.0);
  match (from, to) {
    (Color::Rgb(fr, fg, fb), Color::Rgb(tr, tg, tb)) => Color::Rgb(
      blend_u8_value(fr, tr, t),
      blend_u8_value(fg, tg, t),
      blend_u8_value(fb, tb, t),
    ),
    (a, b) => {
      panic!("failed to blend colors, not RGB values: a = {a:?}, b = {b:?}");
    }
  }
}

/// A material descriptor.
#[allow(clippy::enum_variant_names, dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Material {
  Test,
  WallFace,
  WallEdge,
  WallCorner,
  ColoredEdge(Color),
}

impl Material {
  pub fn draw_request_type(&self) -> MaterialDrawRequestType {
    match self {
      Material::Test => MaterialDrawRequestType::None,
      Material::WallFace => MaterialDrawRequestType::None,
      Material::WallEdge => MaterialDrawRequestType::Neighbors,
      Material::WallCorner => MaterialDrawRequestType::None,
      Material::ColoredEdge(_) => MaterialDrawRequestType::Neighbors,
    }
  }

  pub fn draw(
    &self,
    draw_request: MaterialDrawRequest,
    proj_depth: f32,
  ) -> DrawnMaterial {
    match (self, draw_request) {
      (Material::Test, _) => DrawnMaterial {
        mat: Material::Test,
        sym: "#",
        proj_depth,
      },
      (Material::WallFace, _) => DrawnMaterial {
        mat: Material::WallFace,
        sym: " ",
        proj_depth,
      },
      (Material::WallEdge, MaterialDrawRequest::Neighbors { prev, next }) => {
        DrawnMaterial {
          mat: Material::WallEdge,
          sym: thin_neighbor_symbol(prev, next),
          proj_depth,
        }
      }
      (Material::WallCorner, _) => DrawnMaterial {
        mat: Material::WallCorner,
        sym: "+",
        proj_depth,
      },
      (
        Material::ColoredEdge(color),
        MaterialDrawRequest::Neighbors { prev, next },
      ) => DrawnMaterial {
        mat: Material::ColoredEdge(*color),
        sym: thin_neighbor_symbol(prev, next),
        proj_depth,
      },
      (mat, req) => panic!(
        "material/draw_request mismatch: got material {mat:?}, draw_request \
         {req:?}"
      ),
    }
  }
}

/// The kind of information a given material variant needs to draw itself.
#[derive(Clone, Debug)]
pub enum MaterialDrawRequestType {
  /// No additional information needed.
  None,
  /// This material variant needs its next and previous neighbor directions.
  Neighbors,
}

/// The information a material variant needs to draw itself.
#[derive(Clone, Debug)]
pub enum MaterialDrawRequest {
  /// No additional information needed.
  None,
  /// The previous and next neighbor directions for this cell.
  Neighbors { prev: Neighbor, next: Neighbor },
}

/// A material whose stroke has been determined.
#[derive(Clone, Copy, Debug)]
pub struct DrawnMaterial {
  mat:        Material,
  sym:        &'static str,
  proj_depth: f32,
}

impl DrawnMaterial {
  pub fn render(&self, behind: Option<&Self>) -> Cell {
    let DrawnMaterial {
      mat,
      sym,
      proj_depth,
    } = self;

    match mat {
      Material::Test => {
        let mut cell = Cell::new(sym);
        cell.set_bg(BASE_COLOR_RATATUI);
        cell.set_fg(LINEART_COLOR_RATATUI);
        cell
      }
      Material::WallFace => match behind {
        Some(DrawnMaterial {
          mat: behind_mat,
          sym: behind_symbol,
          ..
        }) => match behind_mat {
          Material::WallFace => {
            let mut cell = Cell::new(sym);
            cell.set_bg(BASE_COLOR_RATATUI);
            cell
          }
          Material::Test | Material::WallEdge | Material::WallCorner => {
            let mut cell = Cell::new(behind_symbol);
            cell.set_bg(BASE_COLOR_RATATUI);
            cell.set_fg(DIM_TEXT_COLOR_RATATUI);
            cell
          }
          Material::ColoredEdge(edge_color) => {
            let mut cell = Cell::new(behind_symbol);
            cell.set_bg(BASE_COLOR_RATATUI);
            cell.set_fg(*edge_color);
            cell
          }
        },
        None => {
          let mut cell = Cell::new(sym);
          cell.set_bg(BASE_COLOR_RATATUI);
          cell
        }
      },
      Material::WallEdge => {
        let mut cell = Cell::new(sym);
        cell.set_bg(BASE_COLOR_RATATUI);
        cell.set_fg(blend_color(
          LINEART_COLOR_RATATUI,
          BASE_COLOR_RATATUI,
          *proj_depth,
        ));
        cell
      }
      Material::WallCorner => {
        let mut cell = Cell::new(sym);
        cell.set_bg(BASE_COLOR_RATATUI);
        cell.set_fg(blend_color(
          LINEART_COLOR_RATATUI,
          BASE_COLOR_RATATUI,
          *proj_depth,
        ));
        cell
      }
      Material::ColoredEdge(color) => {
        let mut cell = Cell::new(sym);
        cell.set_bg(BASE_COLOR_RATATUI);
        cell.set_fg(*color);
        cell
      }
    }
  }
}
