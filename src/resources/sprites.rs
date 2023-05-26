use bytemuck::{Pod, Zeroable};
use wgpu::util::{DeviceExt, BufferInitDescriptor};

use crate::{Perspective, WgpuCore};


pub type SpriteFrameElement = [f32; 2];


pub struct SpriteGpuHandle {
    pub frames: SpriteFramesBuffer,
    pub animation: SpriteAnimationHandle,
    pub timer: SpriteTimerBuffer,
    
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,    
}

impl SpriteGpuHandle {
    pub fn new(device: &wgpu::Device, spriteframes: Vec<SpriteFrameElement>) -> Self {
        let frames = SpriteFramesBuffer::new(device, spriteframes);
        let animation = SpriteAnimationHandle::new(device);
        let timer = SpriteTimerBuffer::new(device);

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Sprite Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sprite Bindgroup"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: frames.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: animation.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: timer.buffer.as_entire_binding(),
                }
            ],
        });

        SpriteGpuHandle {
            frames,
            animation,
            timer,

            layout,
            bindgroup,
        }
    }

    pub fn buffer_update(&mut self, gx: &WgpuCore, frames_passed: u32) {
        self.timer.frames_passed = frames_passed;

        gx.queue.write_buffer(
            &self.timer.buffer, 
            0,
            bytemuck::cast_slice(&[self.timer.frames_passed])
        );
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct SpriteAnimationData {
    start: u32,
    end: u32,
    head: u32,
}

pub struct SpriteAnimationHandle {
    pub buffer: wgpu::Buffer,
}

impl SpriteAnimationHandle {
    pub fn new(device: &wgpu::Device) -> Self {

        let data = SpriteAnimationData { 
            start: 0, end: 3, head: 0
        };

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Sprite Animation Buffer")),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
        });

        SpriteAnimationHandle { buffer }
    }
}


pub struct SpriteFramesBuffer {
    pub buffer: wgpu::Buffer,
}

impl SpriteFramesBuffer {
    pub fn new(device: &wgpu::Device, spriteframes: Vec<SpriteFrameElement>) -> Self {

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Sprite Frames Buffer")),
            contents: bytemuck::cast_slice(spriteframes.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
        });

        SpriteFramesBuffer { buffer }
    }
}


pub struct SpriteTimerBuffer {
    pub frames_passed: u32, // micro seconds
    pub buffer: wgpu::Buffer,
}

impl SpriteTimerBuffer {
    pub fn new(device: &wgpu::Device) -> Self {

        let frames_passed = 0;

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Sprite Timer Buffer")),
            contents: bytemuck::cast_slice(&[frames_passed]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
        });

        SpriteTimerBuffer { buffer, frames_passed }
    }
}