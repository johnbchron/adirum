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

    const DIM_X_COLOR: Color = Color::Rgb(127, 0, 0);
    const DIM_Y_COLOR: Color = Color::Rgb(0, 127, 0);
    const DIM_Z_COLOR: Color = Color::Rgb(0, 0, 127);
    const X_COLOR: Color = Color::Rgb(255, 0, 0);
    const Y_COLOR: Color = Color::Rgb(0, 255, 0);
    const Z_COLOR: Color = Color::Rgb(0, 0, 255);

    let line_style = LineStyle {
      material:     Material::ColoredEdge(DIM_X_COLOR),
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

    line.style.material = Material::ColoredEdge(DIM_Y_COLOR);
    line.style.cap_material = Some(Material::ColoredPoint(Y_COLOR));
    line.to = pos + Vec3::Y * length;
    line.draw(
      self.buffer.buffer_mut(),
      &self.canvas_args,
      &Transform::IDENTITY,
    );

    line.style.material = Material::ColoredEdge(DIM_Z_COLOR);
    line.style.cap_material = Some(Material::ColoredPoint(Z_COLOR));
    line.to = pos + Vec3::Z * length;
    line.draw(
      self.buffer.buffer_mut(),
      &self.canvas_args,
      &Transform::IDENTITY,
    );
  }

  pub fn cornered_box_gizmo(&mut self, pos: Vec3, size: Vec3, color: Color) {
    use crate::shapes::*;

    let line_style = LineStyle {
      material:     Material::ColoredEdge(color),
      cap_material: Some(Material::ColoredPoint(color)),
      variant:      LineVariant::Thin,
    };

    let axis_mask_iter =
      (0..8).map(|i| BVec3::new(i & 1 == 1, i & 2 == 2, i & 4 == 4));

    for axis_mask in axis_mask_iter {
      let direction_mask = Vec3::select(axis_mask, Vec3::ONE, Vec3::NEG_ONE);
      let from = pos + size * direction_mask;

      let to_x = from + direction_mask * Vec3::NEG_X * size.x / 4.0;
      let to_y = from + direction_mask * Vec3::NEG_Y * size.y / 4.0;
      let to_z = from + direction_mask * Vec3::NEG_Z * size.z / 4.0;

      let mut line = LineArgs {
        from,
        to: to_x,
        style: line_style.clone(),
      };

      line.draw(
        self.buffer.buffer_mut(),
        &self.canvas_args,
        &Transform::IDENTITY,
      );

      line.to = to_y;
      line.draw(
        self.buffer.buffer_mut(),
        &self.canvas_args,
        &Transform::IDENTITY,
      );

      line.to = to_z;
      line.draw(
        self.buffer.buffer_mut(),
        &self.canvas_args,
        &Transform::IDENTITY,
      );
    }
  }
}

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
  fn build(&self, app: &mut App) { app.init_resource::<GizmoBuffer>(); }
}
