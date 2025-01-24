use std::{cmp::Ordering, collections::HashMap};

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

impl Ord for UnpositionedDrawnCell {
  fn cmp(&self, other: &Self) -> Ordering {
    let a = self.proj_depth.clamp(0.0, 1.0);
    let b = other.proj_depth.clamp(0.0, 1.0);
    a.partial_cmp(&b).unwrap()
  }
}

impl PartialOrd for UnpositionedDrawnCell {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for UnpositionedDrawnCell {
  fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
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
  pub fn draw(&mut self, mat: DrawnMaterial, position: IVec2, proj_depth: f32) {
    // for now, just throw it away if it's negative
    if position.x < 0 || position.y < 0 {
      return;
    }
    let position = UVec2::new(position.x as _, position.y as _);

    self.buffer.push(DrawnCell {
      mat,
      position,
      proj_depth,
    })
  }

  /// Merges two [`ShapeBuffer`]s.
  pub fn merge(&mut self, mut other: Self) {
    self.buffer.reserve(other.buffer.len());

    self.buffer.append(&mut other.buffer);
  }

  /// Removes every cell deeper than the first two cells in a given position.
  pub fn truncate(self) -> TruncatedShapeBuffer {
    let mut map: HashMap<UVec2, Zot<UnpositionedDrawnCell>> = HashMap::new();

    for cell in self.buffer.into_iter() {
      let DrawnCell {
        mat,
        position,
        proj_depth,
      } = cell;
      let cell = UnpositionedDrawnCell { mat, proj_depth };

      if !(0.0..=1.0).contains(&proj_depth) {
        continue;
      }

      map.entry(position).or_default().add(cell);
    }

    TruncatedShapeBuffer { map }
  }
}

pub struct TruncatedShapeBuffer {
  map: HashMap<UVec2, Zot<UnpositionedDrawnCell>>,
}

impl TruncatedShapeBuffer {
  pub fn render(self, area: Rect) -> Buffer {
    let mut buffer = Buffer::filled(area, DEFAULT_CELL);

    for (pos, zot) in self.map.into_iter() {
      if !area.contains(ratatui::layout::Position {
        x: pos.x as _,
        y: pos.y as _,
      }) {
        continue;
      }

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
      Zot::One(a) => {
        let mut set = [a, value];
        set.sort();
        let [a, b] = set;
        Zot::Two(a, b)
      }
      Zot::Two(a, b) => {
        let mut set = [a, b, value];
        set.sort();
        let [a, b, _] = set;
        Zot::Two(a, b)
      }
    }
  }

  fn add(&mut self, value: T) {
    let val = std::mem::take(self);
    *self = val.add_inner(value);
  }
}
