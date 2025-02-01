use bevy::{prelude::*, utils::HashMap};

use crate::{DEFAULT_BLOCK_HALF_EXTENTS, StationBlock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NavCellCoords(UVec3);

enum NavCellStatus {
  Empty,
  Occupied(Entity),
  Environment,
}

pub struct NavCells {
  grid_origin: Vec3,
  cells:       HashMap<NavCellCoords, NavCellStatus>,
}
