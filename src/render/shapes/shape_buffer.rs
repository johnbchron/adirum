use ratatui::{
  buffer::{Buffer, Cell},
  prelude::Rect,
};

use crate::render::Material;

pub struct ShapeBuffer {
  /// The area of the render buffer that the shape will occupy.
  area:   Rect,
  /// The buffer of cells and their depth values.
  buffer: Vec<(Cell, f32)>,
}

impl ShapeBuffer {
  pub fn new(area: Rect) -> Self {
    Self {
      area,
      buffer: vec![
        (Material::Nothing.to_cell(), 1.0);
        area.area().try_into().unwrap()
      ],
    }
  }

  pub fn area(&self) -> Rect { self.area }

  fn index(&self, x: u16, y: u16) -> Option<usize> {
    self.area.contains((x, y).into()).then(|| {
      if x < self.area.x || y < self.area.y {
        panic!(
          "ShapeBuffer index out of bounds: ({}, {}), area: {}",
          x, y, self.area
        );
      }
      let x = x - self.area.x;
      let y = y - self.area.y;
      (y * self.area.width + x).into()
    })
  }

  pub fn set(&mut self, x: u16, y: u16, cell: Cell, depth: f32) {
    if let Some(idx) = self.index(x, y) {
      // self.buffer[idx] = (cell, depth);
      if depth < self.buffer[idx].1 {
        self.buffer[idx] = (cell, depth);
      }
    }
  }

  pub fn into_buffer(self) -> Buffer {
    Buffer {
      area:    self.area,
      content: self.buffer.into_iter().map(|(cell, _)| cell).collect(),
    }
  }
}
