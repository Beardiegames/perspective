use std::time::Instant;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::{WgpuCore, UniformData};


pub struct RunTime {
    instant: Instant,
    previous: u128,
    elapsed: u128,
    frame_delta: f64,
}

impl RunTime {
    pub fn new() -> Self {
        RunTime {
            instant: Instant::now(),
            previous: 0,
            elapsed: 0,
            frame_delta: 0.0,
        }
    }

    pub fn time_step(&mut self) {
        self.previous = self.elapsed;
        self.elapsed = self.instant.elapsed().as_micros();
        self.frame_delta = (self.elapsed - self.previous) as f64 / 1_000_000.0;
    }

    pub fn elapsed(&self) -> u128 { self.elapsed }

    pub fn frame_delta(&self) -> f64 { self.frame_delta }
}