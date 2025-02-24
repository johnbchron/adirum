use bevy::prelude::*;
use render::{Render, gizmo::Gizmos};

/// Stores a position in block-space.
#[derive(Component, Debug, Default, Clone, Copy)]
#[require(Transform)]
pub struct BlockCoords {
  /// The position of the block in block-space.
  pos: IVec3,
}

impl BlockCoords {
  pub fn new(pos: IVec3) -> Self { Self { pos } }

  pub fn world_space_block_center(
    &self,
    block_transform: Option<&BlockTransform>,
  ) -> Vec3 {
    let center_offset = block_transform
      .map(|bt| bt.center_offset)
      .unwrap_or(Vec3::ZERO);
    self.pos.as_vec3() * DEFAULT_BLOCK_HALF_EXTENTS * 2.0 + center_offset
  }

  fn update_transform(
    &self,
    block_transform: Option<&BlockTransform>,
    transform: &mut Transform,
  ) {
    transform.translation = self.world_space_block_center(block_transform);
    transform.scale = block_transform
      .map(|bt| bt.scale.as_vec3())
      .unwrap_or(Vec3::ONE);
  }
}

/// Stores the local transformation of a block in block-space.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct BlockTransform {
  /// The scale of the block (base block size is [DEFAULT_BLOCK_HALF_EXTENTS]).
  scale:         UVec3,
  /// The offset of the block's center from the block-space position.
  center_offset: Vec3,
}

impl BlockTransform {
  pub fn new(scale: UVec3, center_offset: Vec3) -> Self {
    Self {
      scale,
      center_offset,
    }
  }
}

/// The default block half extents.
///
/// This makes a block that's 4 meters square and 3 meters tall.
pub const DEFAULT_BLOCK_HALF_EXTENTS: Vec3 = Vec3::new(2.0, 1.5, 2.0);

fn update_transforms(
  mut query: Query<(&BlockCoords, Option<&BlockTransform>, &mut Transform)>,
) {
  for (coords, block_transform, mut transform) in query.iter_mut() {
    coords.update_transform(block_transform, &mut transform);
  }
}

fn debug_block_coords(
  mut query: Query<(&BlockCoords, Option<&BlockTransform>)>,
  mut gizmos: Gizmos,
) {
  for (coords, block_transform) in query.iter_mut() {
    gizmos.axis_gizmo(coords.world_space_block_center(block_transform), 0.5);
  }
}

pub struct BlockCoordsPlugin;

impl Plugin for BlockCoordsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PostUpdate, update_transforms)
      .add_systems(Render, debug_block_coords);
  }
}
