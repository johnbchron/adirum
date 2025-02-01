mod block_coords;
mod nav;
mod station_block;

use bevy::prelude::*;

pub use self::{block_coords::*, station_block::*};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((BlockCoordsPlugin, StationBlockPlugin));
  }
}
