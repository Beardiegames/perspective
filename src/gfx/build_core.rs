
use super::*;
use wgpu::{SubmissionIndex, util::DeviceExt, Surface};
use raw_window_handle::*;

impl WgpuCore {

    pub fn new<W>(settings: Option<WindowSettings<W>>) -> anyhow::Result<Self>
        where W: HasRawWindowHandle + HasRawDisplayHandle,
    {

        let instance = wgpu::Instance::new(
                InstanceDescriptor { 
                    backends: wgpu::Backends::all(), 
                    ..Default::default() 
                }
            );
        
        let (mut surface, size) = match settings {
                Some(s) => (
                    unsafe { instance.create_surface(s.window) }.ok(),
                    (
                        if s.width > 50 { s.width } else { 50 }, 
                        if s.height > 50 { s.height } else { 50 }
                    )
                ),
                None => (None, (0, 0))
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
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .block_on()?;
        
        if let Some(srf) = &mut surface {
            let surface_caps = srf.get_capabilities(&adapter);

            let surface_format = surface_caps.formats.iter()
                .copied()
                .filter(|f| f.describe().srgb)
                .next()
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
            srf.configure(&device, &config);
        }
        
        Ok( WgpuCore {
            instance,
            adapter,
            device,
            queue,
            surface,

            bindgroup_count: 0,
        })
    }
}