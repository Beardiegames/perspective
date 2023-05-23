use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::{WgpuCore, RunTime, Camera};


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct UniformData {
    time: u64,
    projection_matrix: [[f32; 4]; 4],
}

impl UniformData {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;

        Self { 
            time: 0, 
            projection_matrix: cgmath::Matrix4::identity().into(),
        }
    }
}

pub struct UniformDataHandle {
    pub data: UniformData,

    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
}

impl UniformDataHandle {
    pub fn new(device: &wgpu::Device) -> Self {
        let data = UniformData::new();

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("camera_buffer"),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        
        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        UniformDataHandle {
            data,

            buffer,
            layout,
            bindgroup,
        }
    }

    pub fn update(&mut self, timer: &RunTime, camera: &Camera) {
        self.data.time = (timer.elapsed() % 1_000_000) as u64;
        self.data.projection_matrix = camera.build_view_map().into();
    }
}