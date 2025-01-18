use std::{
  cmp::Ordering,
  collections::{HashMap, hash_map::Entry},
};

use bevy::math::{IVec2, UVec2};
use ratatui::{buffer::Buffer, layout::Rect};

use super::DrawnMaterial;
use crate::render::DEFAULT_CELL;

/// A single cell which has been drawn by a shape.
struct DrawnCell {
  mat:        DrawnMaterial,
  position:   UVec2,
  proj_depth: f32,
}

/// A drawn cell without its position.
struct UnpositionedDrawnCell {
  mat:        DrawnMaterial,
  proj_depth: f32,
}

impl UnpositionedDrawnCell {
  fn compare(&self, other: &Self) -> Ordering {
    let a = self.proj_depth.clamp(0.0, 1.0);
    let b = other.proj_depth.clamp(0.0, 1.0);
    a.partial_cmp(&b).unwrap()
  }
}

impl Ord for UnpositionedDrawnCell {
  fn cmp(&self, other: &Self) -> Ordering { self.compare(other) }
}

impl PartialOrd for UnpositionedDrawnCell {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.compare(other))
  }
}

impl PartialEq for UnpositionedDrawnCell {
  fn eq(&self, other: &Self) -> bool { self.compare(other) == Ordering::Equal }
}

impl Eq for UnpositionedDrawnCell {}

pub struct ShapeBuffer {
  buffer: Vec<DrawnCell>,
}

impl ShapeBuffer {
  /// Creates a new [`ShapeBuffer`].
  pub fn new() -> Self {
    Self {
      buffer: Vec::with_capacity(100),
    }
  }

  /// Draws a cell.
  pub fn draw(&mut self, cell: DrawnMaterial, position: IVec2, depth: f32) {
    // for now, just throw it away if it's negative
    if position.x < 0 || position.y < 0 {
      return;
    }
    let position = UVec2::new(position.x as _, position.y as _);

    self.buffer.push(DrawnCell {
      mat: cell,
      position,
      proj_depth: depth,
    })
  }

  /// Merges two [`ShapeBuffer`]s.
  pub fn merge(&mut self, mut other: Self) {
    self.buffer.reserve(other.buffer.len());

    self.buffer.append(&mut other.buffer);
  }

  /// Removes every cell deeper than the first two cells in a given position.
  pub fn truncate(self) -> TruncatedShapeBuffer {
    let max_x = self.buffer.iter().fold(0, |a, c| a.max(c.position.x));
    let max_y = self.buffer.iter().fold(0, |a, c| a.max(c.position.y));
    let extent = UVec2::new(max_x, max_y);

    let mut map: HashMap<UVec2, Zot<UnpositionedDrawnCell>> = HashMap::new();

    for cell in self.buffer.into_iter() {
      let DrawnCell {
        mat,
        position,
        proj_depth,
      } = cell;
      let cell = UnpositionedDrawnCell { mat, proj_depth };

      map.entry(position).or_default().add(cell);
    }

    TruncatedShapeBuffer { map, extent }
  }
}

pub struct TruncatedShapeBuffer {
  map:    HashMap<UVec2, Zot<UnpositionedDrawnCell>>,
  extent: UVec2,
}

impl TruncatedShapeBuffer {
  pub fn render(self) -> Buffer {
    let mut buffer = Buffer::filled(
      Rect {
        x:      0,
        y:      0,
        width:  (self.extent.x + 1) as _,
        height: (self.extent.y + 1) as _,
      },
      DEFAULT_CELL,
    );

    for (pos, zot) in self.map.into_iter() {
      let cell = buffer.cell_mut((pos.x as u16, pos.y as u16)).unwrap();
      match zot {
        Zot::Zero => continue,
        Zot::One(a) => {
          *cell = a.mat.render(None);
        }
        Zot::Two(a, b) => {
          *cell = a.mat.render(Some(&b.mat));
        }
      }
    }

    buffer
  }
}

#[derive(Clone, Default)]
enum Zot<T> {
  #[default]
  Zero,
  One(T),
  Two(T, T),
}

impl<T: Ord> Zot<T> {
  /// Keeps the lowest 2 values (according to `Ord`) when it already has 2.
  fn add_inner(self, value: T) -> Self {
    match self {
      Zot::Zero => Zot::One(value),
      Zot::One(a) => match a.cmp(&value) {
        Ordering::Less => Zot::Two(a, value),
        Ordering::Equal => Zot::Two(a, value),
        Ordering::Greater => Zot::Two(value, a),
      },
      Zot::Two(a, b) => {
        debug_assert!(matches!(a.cmp(&b), Ordering::Less | Ordering::Equal));
        match a.cmp(&value) {
          Ordering::Less => match b.cmp(&value) {
            Ordering::Less => Zot::Two(a, b),
            Ordering::Equal => Zot::Two(a, b),
            Ordering::Greater => Zot::Two(a, value),
          },
          Ordering::Equal => Zot::Two(a, value),
          Ordering::Greater => Zot::Two(value, a),
        }
      }
    }
  }

  fn add(&mut self, value: T) {
    let val = std::mem::take(self);
    *self = val.add_inner(value);
  }
}
