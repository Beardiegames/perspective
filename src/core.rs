//use anyhow::Error;
use raw_window_handle::*;
use pollster::FutureExt;
use wgpu::{
    Surface, Adapter, Device,
    Dx12Compiler, InstanceDescriptor,
};
use crate::interface::RenderContext;

pub use crate::renderer::*;
pub use crate::resources::*;


pub struct WindowSettings<'a, W>
    where W: HasRawWindowHandle + HasRawDisplayHandle,
{
    pub window: &'a W,
    pub width: u32,
    pub height: u32,
    pub camera: CameraSetup,
}


pub struct Canvas {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub depth_map: DepthTexture,
}


pub struct WgpuCore {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub canvas: Option<Canvas>,
    pub timer: RunTime,
}


impl WgpuCore {

    pub fn new<W>(settings: Option<&WindowSettings<W>>) -> anyhow::Result<Self>
        where W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let instance = wgpu::Instance::new(WGPU_INSTANCE_DESCRIPTOR);
        
        let (surface, size, camera) = match settings {
                Some(s) => (
                    unsafe { instance.create_surface(&s.window) }.ok(),
                    (s.width.clamp(0, 50), s.height.clamp(0, 50)),
                    s.camera.clone(),
                ),
                None => (None, (0, 0), CameraSetup::default())
            };

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

        Ok( WgpuCore {
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
    
    // pub fn setup_render_processor(&mut self, camera_setup: &CameraSetup, assets: AssetPack) -> Renderer {
    //     Renderer::new(
    //         &self.device, 
    //         //&self.queue, 
    //         camera_setup,
    //         assets
    //     )
    // }
}


// -- helpers --

const WGPU_INSTANCE_DESCRIPTOR: InstanceDescriptor = InstanceDescriptor { 
    backends: wgpu::Backends::all(),
    dx12_shader_compiler: Dx12Compiler::Fxc,
};

fn create_canvas(surface: Option<Surface>, device: &Device, adapter: &Adapter, size: (u32, u32)) -> Option<Canvas> {
    surface.map(|srf| {
        let surface_caps = srf.get_capabilities(adapter);

        let surface_format = surface_caps.formats.iter()
            .copied().find(|f| f.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        srf.configure(device, &config);

        let depth_map = DepthTexture::new(device, &config);

        Canvas {  
            surface: srf,
            config,
            depth_map
        }
    })
}
