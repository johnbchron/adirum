use bevy::{ecs::system::SystemParam, prelude::*};

use crate::shapes::{CanvasArgs, ShapeBuffer};

#[derive(Resource, Default)]
pub struct GizmoBuffer {
  buffer: ShapeBuffer,
}

impl GizmoBuffer {
  pub fn buffer_mut(&mut self) -> &mut ShapeBuffer { &mut self.buffer }
}

#[derive(SystemParam)]
pub struct Gizmos<'w> {
  buffer:      ResMut<'w, GizmoBuffer>,
  canvas_args: CanvasArgs<'w>,
}

impl<'w> Gizmos<'w> {
  pub fn canvas_args(&self) -> CanvasArgs<'w> { self.canvas_args.clone() }
  pub fn buffer_mut(&'w mut self) -> &'w mut ShapeBuffer {
    self.buffer.buffer_mut()
  }
}

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
  fn build(&self, app: &mut App) { app.init_resource::<GizmoBuffer>(); }
}
