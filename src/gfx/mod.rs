mod core;

use pollster::FutureExt;
use wgpu::InstanceDescriptor;


pub enum PipelineHandle {
    Compute (wgpu::ComputePipeline),
}

pub struct BufferHandle {
    staging: wgpu::Buffer,
    storage: wgpu::Buffer,
}

pub struct BindgroupHandle {
    bind_group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    set_idx: u32,
    bond_idx: u32,
}

pub struct ShaderHandle {
    shader: wgpu::ShaderModule,
    pipeline: PipelineHandle,
}

pub struct WgpuCore {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    bindgroup_count: u32,
}