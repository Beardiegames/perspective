use bytemuck::{Pod, Zeroable};
use cgmath::Rotation3;
use wgpu::util::DeviceExt;

use crate::WgpuCore;


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    _padding: u32, // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here

    pub color: [f32; 3],
    _padding2: u32,

    pub ambient: [f32; 3],
    _padding3: u32,
}

pub struct Light {
    pub uniform: LightUniform,

    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
}

impl Light {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform = LightUniform {
            position: [2.0, 1.0, 2.0],
            _padding: 0,

            color: [0.95, 0.35, 0.25],
            _padding2: 0,

            ambient: [0.03, 0.05, 0.075],
            _padding3: 0,
        };
        
        // We'll want to update our lights position, so we use COPY_DST
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        Light {
            uniform,
            
            buffer,
            layout,
            bindgroup,
        }
    }

    pub fn buffer_update(&mut self, gx: &WgpuCore) {
        // let old_position: cgmath::Vector3<_> = self.uniform.position.into();

        // let new_position = cgmath::Quaternion::from_axis_angle(
        //         (0.0, 1.0, 0.0).into(), 
        //         cgmath::Deg(1.0)
        //     ) * old_position;

        // self.uniform.position = new_position.into();

        gx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

}