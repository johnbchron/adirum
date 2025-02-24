use bevy::prelude::*;
use render::{
  Render,
  shapes::{CanvasArgs, RenderedShape},
};

use crate::{BlockTransform, DEFAULT_BLOCK_HALF_EXTENTS};

#[derive(Component)]
#[require(RenderedShape, Transform)]
pub enum StationBlockType {
  Room,
}

impl StationBlockType {
  pub fn block_transform(&self) -> BlockTransform {
    match self {
      StationBlockType::Room => {
        BlockTransform::new(UVec3::new(1, 1, 1), Vec3::ZERO)
      }
    }
  }
}

fn render_station_block(
  canvas_args: CanvasArgs,
  mut query: Query<(&Transform, &StationBlockType, &mut RenderedShape)>,
) {
  use render::shapes::*;

  for (transform, block, mut buffer) in query.iter_mut() {
    match block {
      StationBlockType::Room => {
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

        cuboid.draw(buffer.inner_mut(), &canvas_args, transform);
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
