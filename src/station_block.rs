use bevy::prelude::*;

use crate::{
  block_coords::DEFAULT_BLOCK_HALF_EXTENTS,
  render::{
    Render,
    shapes::{CanvasArgs, ShapeBuffer},
  },
};

#[derive(Component)]
#[require(ShapeBuffer)]
pub enum StationBlock {
  Room,
}

fn render_station_block(
  canvas_args: CanvasArgs,
  mut query: Query<(&Transform, &StationBlock, &mut ShapeBuffer)>,
) {
  use crate::render::shapes::*;

  for (transform, block, mut buffer) in query.iter_mut() {
    match block {
      StationBlock::Room => {
        let cuboid_style = CuboidStyle {
          line_material:   Material::WallEdge,
          corner_material: Some(Material::WallCorner),
          face_material:   None,
          line_variant:    LineVariant::Thin,
        };

        let cuboid = CuboidArgs {
          half_extents: DEFAULT_BLOCK_HALF_EXTENTS,
          style:        cuboid_style,
        };

        cuboid.draw(&mut buffer, &canvas_args, transform);
      }
    }
  }
}

pub struct StationBlockPlugin;

impl Plugin for StationBlockPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Render, render_station_block);
  }
}
