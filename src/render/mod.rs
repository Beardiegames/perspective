pub mod sprites;
pub mod buffer;
pub mod tiles;

use macroquad::prelude::*;
pub use sprites::*;
pub use buffer::*;
pub use tiles::*;


pub fn build_pipeline() -> PipelineParams {
    PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    }
}
