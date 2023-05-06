// use pollster::FutureExt;
// use wgpu::{InstanceDescriptor};
use raw_window_handle::*;

pub use crate::processors::*;
pub use crate::resources::*;


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

    pub fn setup_compute_processor<T>(&mut self, settings: &ComputeSettings<T>) -> ComputeProcessor
        where T: bytemuck::Pod + Clone
    {
        ComputeProcessor::new(self, settings)
    }

    pub fn setup_render_processor(&mut self, settings: &RenderSettings) -> RenderProcessor {
        RenderProcessor::new(self, settings)
    }
}
