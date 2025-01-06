use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Neighbor {
  TopLeft,
  Top,
  TopRight,
  Left,
  Right,
  BottomLeft,
  Bottom,
  BottomRight,
}

use Neighbor::*;

impl Neighbor {
  pub fn find(offset: IVec2) -> Neighbor {
    let norm_offset = offset.as_vec2().normalize_or(Vec2::Y).round().as_ivec2();

    // this is in canvas coords as ratatui shows them in its buffers, so +x+y is
    // bottom right
    match (norm_offset.x, norm_offset.y) {
      (-1, -1) => TopLeft,
      (0, -1) => Top,
      (1, -1) => TopRight,
      (-1, 0) => Left,
      // no middle
      (1, 0) => Right,
      (-1, 1) => BottomLeft,
      (0, 1) => Bottom,
      (1, 1) => BottomRight,
      _ => panic!(
        "failed to find neighbor for coordinates {offset}: got {norm_offset} \
         from rounding"
      ),
    }
  }
}

pub fn thin_neighbor_symbol(from: Neighbor, to: Neighbor) -> &'static str {
  match (from, to) {
    (TopLeft, TopLeft) => "`",
    (TopLeft, Top) => "*",
    (TopLeft, TopRight) => "*",
    (TopLeft, Left) => "⦣",
    (TopLeft, Right) => "`",
    (TopLeft, BottomLeft) => ")",
    (TopLeft, Bottom) => "⎞",
    (TopLeft, BottomRight) => "\\",
    (Top, TopLeft) => thin_neighbor_symbol(to, from),
    (Top, Top) => "*",
    (Top, TopRight) => "´",
    (Top, Left) => "*",
    (Top, Right) => "*",
    (Top, BottomLeft) => "⎠",
    (Top, Bottom) => "|",
    (Top, BottomRight) => "⎝",
    (TopRight, TopLeft) => thin_neighbor_symbol(to, from),
    (TopRight, Top) => thin_neighbor_symbol(to, from),
    (TopRight, TopRight) => "´",
    (TopRight, Left) => "*",
    (TopRight, Right) => "∠",
    (TopRight, BottomLeft) => "/",
    (TopRight, Bottom) => "⎛",
    (TopRight, BottomRight) => "(",
    (Left, TopLeft) => thin_neighbor_symbol(to, from),
    (Left, Top) => thin_neighbor_symbol(to, from),
    (Left, TopRight) => thin_neighbor_symbol(to, from),
    (Left, Left) => "-",
    (Left, Right) => "-",
    (Left, BottomLeft) => "7",
    (Left, Bottom) => ".",
    (Left, BottomRight) => ".",
    (Right, TopLeft) => thin_neighbor_symbol(to, from),
    (Right, Top) => thin_neighbor_symbol(to, from),
    (Right, TopRight) => thin_neighbor_symbol(to, from),
    (Right, Left) => thin_neighbor_symbol(to, from),
    (Right, Right) => "-",
    (Right, BottomLeft) => "?",
    (Right, Bottom) => ".",
    (Right, BottomRight) => ".",
    (BottomLeft, TopLeft) => thin_neighbor_symbol(to, from),
    (BottomLeft, Top) => thin_neighbor_symbol(to, from),
    (BottomLeft, TopRight) => thin_neighbor_symbol(to, from),
    (BottomLeft, Left) => thin_neighbor_symbol(to, from),
    (BottomLeft, Right) => thin_neighbor_symbol(to, from),
    (BottomLeft, BottomLeft) => ".",
    (BottomLeft, Bottom) => ".",
    (BottomLeft, BottomRight) => "‸",
    (Bottom, TopLeft) => thin_neighbor_symbol(to, from),
    (Bottom, Top) => thin_neighbor_symbol(to, from),
    (Bottom, TopRight) => thin_neighbor_symbol(to, from),
    (Bottom, Left) => thin_neighbor_symbol(to, from),
    (Bottom, Right) => thin_neighbor_symbol(to, from),
    (Bottom, BottomLeft) => thin_neighbor_symbol(to, from),
    (Bottom, Bottom) => ".",
    (Bottom, BottomRight) => ".",
    (BottomRight, TopLeft) => thin_neighbor_symbol(to, from),
    (BottomRight, Top) => thin_neighbor_symbol(to, from),
    (BottomRight, TopRight) => thin_neighbor_symbol(to, from),
    (BottomRight, Left) => thin_neighbor_symbol(to, from),
    (BottomRight, Right) => thin_neighbor_symbol(to, from),
    (BottomRight, BottomLeft) => thin_neighbor_symbol(to, from),
    (BottomRight, Bottom) => thin_neighbor_symbol(to, from),
    (BottomRight, BottomRight) => ".",
  }
}
