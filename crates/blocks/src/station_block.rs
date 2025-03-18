use bevy::prelude::*;
use render::{
  Render,
  shapes::{CanvasArgs, RenderedShape},
};

use crate::{BlockTransform, DEFAULT_BLOCK_HALF_EXTENTS};

#[derive(Component, Reflect)]
#[require(RenderedShape, BlockTransform)]
pub enum StationBlockType {
  Room,
  QuadRoomXZ,
}

impl StationBlockType {
  pub fn block_transform(&self) -> BlockTransform {
    match self {
      StationBlockType::Room => {
        BlockTransform::new(UVec3::new(1, 1, 1), Vec3::ZERO)
      }
      StationBlockType::QuadRoomXZ => {
        BlockTransform::new(UVec3::new(2, 1, 2), Vec3::ZERO)
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
      StationBlockType::Room | StationBlockType::QuadRoomXZ => {
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
    app
      .register_type::<StationBlockType>()
      .add_systems(Render, render_station_block);
  }
}
