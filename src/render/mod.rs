pub mod sprites;
pub mod buffer;
pub mod tiles;
pub mod scenes;
pub mod gui;

use macroquad::prelude::*;
pub use sprites::*;
pub use buffer::*;
pub use tiles::*;
pub use scenes::*;
pub use gui::*;
use super::*;


pub fn build_gl_pipeline() -> PipelineParams {
    PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    }
}
