use raw_window_handle::*;
use pollster::FutureExt;
use wgpu::{Surface, Adapter, Device, Dx12Compiler, InstanceDescriptor};

pub use crate::renderer::*;
pub use crate::resources::*;
pub use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop}, 
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};

const WGPU_INSTANCE_DESCRIPTOR: InstanceDescriptor = InstanceDescriptor { 
    backends: wgpu::Backends::all(),
    dx12_shader_compiler: Dx12Compiler::Fxc,
    flags: wgpu::InstanceFlags::DEBUG,
    gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
};

pub struct Canvas {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub depth_map: DepthTexture,
}

pub struct WgpuGrapics {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub canvas: Option<Canvas>,
    pub timer: RunTime,
}

impl WgpuGrapics {

    pub fn new(window: &Window, size: PhysicalSize<u32>) -> anyhow::Result<Self>
        where Window: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let instance = wgpu::Instance::new(WGPU_INSTANCE_DESCRIPTOR);
        let surface = unsafe { instance.create_surface(window) }.ok();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: surface.as_ref(),
            })
            .block_on()
            .ok_or(anyhow::anyhow!("Couldn't create the adapter"))?;
            
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .block_on()?;
        
        let canvas = create_canvas(surface, &device, &adapter, size);         

        Ok( WgpuGrapics {
            instance,
            adapter,
            device,
            queue,
            canvas,
            timer: RunTime::new(),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 { return; } 
        
        if let Some(c) = &mut self.canvas {
            c.config.width = width;
            c.config.height = height;
            c.surface.configure(&self.device, &c.config);
            c.depth_map = DepthTexture::new(&self.device, &c.config);
        }
    }
}

fn create_canvas(surface: Option<Surface>, device: &Device, adapter: &Adapter, size: PhysicalSize<u32>) -> Option<Canvas> {
    surface.map(|srf| {
        // let surface_caps = srf.get_capabilities(adapter);

        // let surface_format = surface_caps.formats.iter()
        //     .copied().find(|f| f.describe().srgb)
        //     .unwrap_or(surface_caps.formats[0]);

        // let config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: surface_format,
        //     width: size.width,
        //     height: size.height,
        //     present_mode: surface_caps.present_modes[0],
        //     alpha_mode: surface_caps.alpha_modes[0],
        //     view_formats: vec![],
        // };
        let config = srf.get_default_config(adapter, size.width, size.height).unwrap();
        srf.configure(device, &config);

        let depth_map = DepthTexture::new(device, &config);

        Canvas {  
            surface: srf,
            config,
            depth_map
        }
    })
}
