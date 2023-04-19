pub mod corebuilder;
//pub mod encoders;
mod pipeline;
mod bindgroups;
mod buffers;
mod compute;

use pollster::FutureExt;
use wgpu::{InstanceDescriptor, BindGroupLayout};
use raw_window_handle::*;
use pipeline::*;
use bindgroups::*;
use buffers::*;

pub use compute::*;


pub struct WindowSettings<'a, W>
    where W: HasRawWindowHandle + HasRawDisplayHandle,
{
    pub window: &'a W,
    pub width: u32,
    pub height: u32,
}

pub struct Canvas {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
}

pub struct WgpuCore {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub canvas: Option<Canvas>,

    bindgroup_count: u32,
}

impl WgpuCore {
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 { return; } 
        
        if let Some(c) = &mut self.canvas {
            c.config.width = width;
            c.config.height = height;
            c.surface.configure(&self.device, &c.config);
        }
    }
}
