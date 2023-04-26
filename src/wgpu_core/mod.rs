pub mod corebuilder;
//mod pipeline;
//mod bindgroups;
//mod compute_buffer;
mod compute;
mod render;

use pollster::FutureExt;
use wgpu::{InstanceDescriptor};
use raw_window_handle::*;

//pub use pipeline::*;
//pub use bindgroups::*;
//pub use compute_buffer::*;
pub use compute::*;
pub use render::*;


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

    pub fn quick_inject_render_passes(&self, view: & wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }
}
