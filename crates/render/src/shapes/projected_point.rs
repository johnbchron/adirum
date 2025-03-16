use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct ProjectedPoint(IVec2, f32);

impl ProjectedPoint {
  pub fn new(pos: IVec2, depth: f32) -> ProjectedPoint {
    ProjectedPoint(pos, depth)
  }

  pub fn pos(&self) -> IVec2 { self.0 }
  pub fn depth(&self) -> f32 { self.1 }

  pub fn set_depth(&mut self, new: f32) { self.1 = new; }
}
