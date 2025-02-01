use bevy::{ecs::system::SystemParam, prelude::*};
use ratatui::prelude::Color;

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

impl Gizmos<'_> {
  pub fn axis_gizmo(&mut self, pos: Vec3, length: f32) {
    use crate::shapes::*;

    const X_COLOR: Color = Color::Rgb(255, 0, 0);
    const Y_COLOR: Color = Color::Rgb(0, 255, 0);
    const Z_COLOR: Color = Color::Rgb(0, 0, 255);

    let line_style = LineStyle {
      material:     Material::ColoredEdge(X_COLOR),
      cap_material: Some(Material::ColoredPoint(X_COLOR)),
      variant:      LineVariant::Thin,
    };

    let mut line = LineArgs {
      from:  pos,
      to:    pos + Vec3::X * length,
      style: line_style,
    };

    line.draw(
      self.buffer.buffer_mut(),
      &self.canvas_args,
      &Transform::IDENTITY,
    );

    line.style.material = Material::ColoredEdge(Y_COLOR);
    line.style.cap_material = Some(Material::ColoredPoint(Y_COLOR));
    line.to = pos + Vec3::Y * length;
    line.draw(
      self.buffer.buffer_mut(),
      &self.canvas_args,
      &Transform::IDENTITY,
    );

    line.style.material = Material::ColoredEdge(Z_COLOR);
    line.style.cap_material = Some(Material::ColoredPoint(Z_COLOR));
    line.to = pos + Vec3::Z * length;
    line.draw(
      self.buffer.buffer_mut(),
      &self.canvas_args,
      &Transform::IDENTITY,
    );
  }
}

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
  fn build(&self, app: &mut App) { app.init_resource::<GizmoBuffer>(); }
}
