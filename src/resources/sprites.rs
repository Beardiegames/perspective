use std::{f32::consts::PI};
use bytemuck::{Pod, Zeroable};
use crate::{WgpuCore, create_sprite_animation_binding, WgpuBinding};



#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct SpriteAnimationData {
    frames: [u32; 2],
    offset: u32,
    head: u32,
}

pub type SpriteFrameElement = [f32; 2];


pub struct SpriteGpuHandle {
    _spriteframes: Vec<SpriteFrameElement>,
    _animations: Vec::<SpriteAnimationData>,
    frames_passed: u32, 

    pub binding: WgpuBinding,
}

impl SpriteGpuHandle {
    pub fn new(
        device: &wgpu::Device,
        layout:&wgpu::BindGroupLayout,
        spriteframes: Vec<SpriteFrameElement>, 
        num_instances: usize
    ) -> Self {
        let mut animations = Vec::<SpriteAnimationData>::new();
        let frames_passed = 0;

        for i in 0..num_instances {
            animations.push(SpriteAnimationData { 
                frames: [0, 3], 
                head: 0,
                offset: (i as f32 * PI) as u32 % 10,
            });
        }

        let binding = create_sprite_animation_binding(device, layout, &animations, &spriteframes, frames_passed);

        SpriteGpuHandle {
            _spriteframes: spriteframes,
            _animations: animations,
            frames_passed,

            binding,
        }
    }

    pub fn buffer_update(&mut self, gx: &WgpuCore, frames_passed: u32) {
        self.frames_passed = frames_passed;

        gx.queue.write_buffer(
            &self.binding.buffers[1], 
            0,
            bytemuck::cast_slice(&[self.frames_passed])
        );
    }
}