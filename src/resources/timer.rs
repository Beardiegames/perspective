use std::time::Instant;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;


pub struct RunTime {
    instant: Instant,
    previous: u128,
    elapsed: u128,
    frame_delta: f64,

    pub animation_speed: u128, // animation framerate in milli-seconds
    animation_frame: u32,
}

impl RunTime {
    pub fn new() -> Self {
        RunTime {
            instant: Instant::now(),
            previous: 0,
            elapsed: 0,
            frame_delta: 0.0,

            animation_speed: 80, // 80ms = 12.5 fps
            animation_frame: 0,
        }
    }

    pub fn time_step(&mut self) {
        self.previous = self.elapsed;
        self.elapsed = self.instant.elapsed().as_micros();
        self.frame_delta = (self.elapsed - self.previous) as f64 / 1_000_000.0;
        self.animation_frame = (self.elapsed / (self.animation_speed * 1_000)) as u32;
    }

    pub fn elapsed(&self) -> u128 { self.elapsed }

    pub fn frame_delta(&self) -> f64 { self.frame_delta }

    pub fn animation_frame(&self) -> u32 { self.animation_frame }
}