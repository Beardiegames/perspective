use std::time::Instant;


pub struct RunTime {
    instant: Instant,
    previous: u128,
    elapsed: u128,
    elapsed_avg: u128,

    frame_delta: f32,
    frame_micros: u32,

    sprite_frames: u32,
    sprite_counter: u32,
    sprite_delay: u32,
}

impl RunTime {
    pub fn new() -> Self {
        RunTime {
            instant: Instant::now(),
            previous: 0,
            elapsed: 0,
            elapsed_avg: 0,
            frame_delta: 0.0,
            frame_micros: 0,

            sprite_frames: 0,
            sprite_counter: 0,
            sprite_delay: 70_000,
        }
    }

    pub fn time_step(&mut self) {
        self.previous = self.elapsed_avg;
        self.elapsed = self.instant.elapsed().as_micros();
        self.elapsed_avg = (self.elapsed_avg + self.elapsed) / 2;

        self.frame_micros = (self.elapsed_avg - self.previous) as u32;
        self.frame_delta = self.frame_micros as f32 / 1_000_000.0;

        self.sprite_counter += self.frame_micros;

        if self.sprite_counter > self.sprite_delay {
            let steps = self.sprite_counter / self.sprite_delay;
            self.sprite_counter -= steps * self.sprite_delay;
            self.sprite_frames += steps;
        }
    }

    pub fn elapsed(&self) -> u128 { self.elapsed_avg }

    pub fn frame_micros(&self) -> u32 { self.frame_micros }
    pub fn frame_millis(&self) -> u32 { self.frame_micros / 1_000 }
    pub fn frame_delta(&self) -> f32 { self.frame_delta }

    pub fn sprite_frames(&self) -> u32 { self.sprite_frames }
}