mod build_core;
pub mod encoders;
mod pipeline;
mod bindgroups;
mod buffers;

use pollster::FutureExt;
use wgpu::{InstanceDescriptor, BindGroupLayout};
use raw_window_handle::*;

pub struct BufferHandle<'a, T> {
    data: &'a [T],
    size: u64,
    staging: wgpu::Buffer,
    storage: wgpu::Buffer,
}

pub struct BindgroupHandle {
    bind_group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    set_idx: u32,
    bond_idx: u32,
}

impl BindgroupHandle {
    pub fn index(&self) -> &u32 { &self.set_idx }
    // pub fn set_idx(&self) -> &u32 { &self.set_idx }
    // pub fn bond_idx(&self) -> &u32 { &self.bond_idx }
}

pub trait PipelineHandle {
    fn get_bind_group_layout(&self, set_idx: u32) -> BindGroupLayout;
}

pub struct ComputePipeHandle {
    shader: wgpu::ShaderModule,
    pipeline: wgpu::ComputePipeline,
}

impl PipelineHandle for ComputePipeHandle {
    fn get_bind_group_layout(&self, idx: u32) -> BindGroupLayout {
        self.pipeline.get_bind_group_layout(idx) 
    }
}

pub struct WindowSettings<'a, W>
    where W: HasRawWindowHandle + HasRawDisplayHandle,
{
    pub window: &'a W,
    pub width: u32,
    pub height: u32
}

pub struct WgpuCore {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: Option<wgpu::Surface>,

    bindgroup_count: u32,
}
