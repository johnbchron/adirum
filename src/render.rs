pub mod camera;
pub mod render_buffer;

#[derive(Clone)]
pub enum Material {
  Wall,
  WallCorner,
  Nothing,
}
