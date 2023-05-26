use bytemuck::{Pod, Zeroable};
use wgpu::util::{DeviceExt, BufferInitDescriptor};

use crate::{WgpuCore, RunTime, Camera};


pub type SpriteFrameElement = [f32; 2];


pub struct SpriteGpuHandle {
    pub frames: SpriteFrameHandle,
    pub animation: SpriteAnimationHandle,
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,    
}

impl SpriteGpuHandle {
    pub fn new(device: &wgpu::Device, spriteframes: Vec<SpriteFrameElement>) -> Self {
        let frames = SpriteFrameHandle::new(device, spriteframes);
        let animation = SpriteAnimationHandle::new(device);

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
                }
            ],
        });

        SpriteGpuHandle {
            frames,
            animation,
            layout,
            bindgroup,
        }
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct SpriteAnimationData {
    start: u32,
    end: u32,
    head: u32,

    delay: u32,
    count: u32,
}

pub struct SpriteAnimationHandle {
    pub buffer: wgpu::Buffer,
}

impl SpriteAnimationHandle {
    pub fn new(device: &wgpu::Device) -> Self {

        let data = SpriteAnimationData { 
            start: 0, end: 3, head: 0, 
            delay: 60, count: 0 
        };
        //let size = std::mem::size_of::<SpriteFrameElement>() as wgpu::BufferAddress;

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Sprite Animation Buffer")),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
        });

        SpriteAnimationHandle {
            buffer,
        }
    }
}


pub struct SpriteFrameHandle {
    //pub cellcount: u32,
    //pub size: u64,
    //pub chunksize: usize,
    pub buffer: wgpu::Buffer,
}

impl SpriteFrameHandle {
    pub fn new(device: &wgpu::Device, spriteframes: Vec<SpriteFrameElement>) -> Self {

        //let framecount = spriteframes.len();
        //let chunksize = std::mem::size_of::<SpriteFrameElement>();
        //let slice_size = framecount * chunksize;
        //let size = slice_size as wgpu::BufferAddress;

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Sprite Animation Buffer")),
            contents: bytemuck::cast_slice(spriteframes.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
        });

        SpriteFrameHandle {
            //cellcount: framecount as u32,
            //size,
            //chunksize,
            buffer,
        }
    }
}