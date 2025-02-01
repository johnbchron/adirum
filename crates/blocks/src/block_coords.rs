use bevy::prelude::*;

/// Stores a position in "block coordinates".
#[derive(Component, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[require(Transform)]
pub struct BlockCoords(IVec3);

impl BlockCoords {
  pub fn new(coords: IVec3) -> Self { Self(coords) }
  fn as_vec3(&self) -> Vec3 {
    Vec3::new(self.0.x as f32, self.0.y as f32, self.0.z as f32)
  }
}

/// The default block half extents.
///
/// This makes a block that's 4 meters square and 3 meters tall.
pub const DEFAULT_BLOCK_HALF_EXTENTS: Vec3 = Vec3::new(2.0, 1.5, 2.0);

#[derive(Resource)]
pub struct BlockCoordsGrid {
  block_half_extents: Vec3,
  origin:             Vec3,
}

impl BlockCoordsGrid {
  pub fn get_block_center(&self, coords: BlockCoords) -> Vec3 {
    self.origin + (coords.as_vec3() * self.block_half_extents * 2.0)
  }
}

impl Default for BlockCoordsGrid {
  fn default() -> Self {
    Self {
      block_half_extents: DEFAULT_BLOCK_HALF_EXTENTS,
      origin:             Vec3::ZERO,
    }
  }
}

fn update_transforms(
  mut query: Query<(&BlockCoords, &mut Transform)>,
  settings: Res<BlockCoordsGrid>,
) {
  for (coords, mut transform) in query.iter_mut() {
    transform.translation = settings.get_block_center(*coords);
    transform.scale = DEFAULT_BLOCK_HALF_EXTENTS;
  }
}

pub struct BlockCoordsPlugin;

impl Plugin for BlockCoordsPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<BlockCoordsGrid>()
      .add_systems(PostUpdate, update_transforms);
  }
}
