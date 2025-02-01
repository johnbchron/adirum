use bevy::prelude::*;

/// Stores a position in "block coordinates".
#[derive(Component, Debug, Default, Clone, Copy)]
#[require(Transform)]
pub struct BlockCoords {
  /// The position of the block in block-space.
  pos:           IVec3,
  /// The scale of the block (base block size is [DEFAULT_BLOCK_HALF_EXTENTS]).
  scale:         UVec3,
  /// The offset of the block's center from the block-space position.
  center_offset: Vec3,
}

impl BlockCoords {
  pub fn new(pos: IVec3, scale: UVec3, center_offset: Vec3) -> Self {
    Self {
      pos,
      scale,
      center_offset,
    }
  }

  pub fn new_single(pos: IVec3) -> Self {
    Self {
      pos,
      scale: UVec3::ONE,
      center_offset: Vec3::ZERO,
    }
  }

  fn world_space_block_center(&self) -> Vec3 {
    self.pos.as_vec3() * DEFAULT_BLOCK_HALF_EXTENTS * 2.0 + self.center_offset
  }

  fn update_transform(&self, transform: &mut Transform) {
    transform.translation = self.world_space_block_center();
    transform.scale = self.scale.as_vec3() * DEFAULT_BLOCK_HALF_EXTENTS;
  }
}

/// The default block half extents.
///
/// This makes a block that's 4 meters square and 3 meters tall.
pub const DEFAULT_BLOCK_HALF_EXTENTS: Vec3 = Vec3::new(2.0, 1.5, 2.0);

fn update_transforms(mut query: Query<(&BlockCoords, &mut Transform)>) {
  for (coords, mut transform) in query.iter_mut() {
    coords.update_transform(&mut transform);
  }
}

pub struct BlockCoordsPlugin;

impl Plugin for BlockCoordsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PostUpdate, update_transforms);
  }
}
