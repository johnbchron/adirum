use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct ProjectedPoint(IVec2, f32);

impl ProjectedPoint {
  pub fn new(pos: IVec2, depth: f32) -> ProjectedPoint {
    debug_assert!(depth - f32::EPSILON <= 1.0);
    debug_assert!(depth + f32::EPSILON >= 0.0);
    ProjectedPoint(pos, depth)
  }

  pub fn pos(&self) -> IVec2 { self.0 }
  pub fn depth(&self) -> f32 { self.1 }

  pub fn set_depth(&mut self, new: f32) {
    debug_assert!(new - f32::EPSILON <= 1.0);
    debug_assert!(new + f32::EPSILON >= 0.0);
    self.1 = new;
  }
}
