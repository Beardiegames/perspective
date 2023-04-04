use pollster::FutureExt;
use wgpu::InstanceDescriptor;


struct BufferHandles {
    staging_buffer: wgpu::Buffer,
    storage_buffer: wgpu::Buffer,
}

enum PipelineHandle {
    Compute (wgpu::ComputePipeline),
}

struct ShaderHandles {
    shader: wgpu::ShaderModule,
    pipeline: PipelineHandle,
}

struct WgpuCore {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl WgpuCore {
    fn init() -> Result<WgpuCore, ()> {
        let instance = wgpu::Instance::new(
                InstanceDescriptor { 
                    backends: wgpu::Backends::all(), 
                    ..Default::default() 
                }
            );
            
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .block_on()
            .ok_or(anyhow::anyhow!("Couldn't create the adapter"))
            .map_err(|e| ())?;
            
        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .block_on()
            .map_err(|e| ())?;
        
        Ok( WgpuCore {
            instance,
            adapter,
            device,
            queue,
        })
    }
    
    fn setup_compute_pipeline(&self, label: &str, shader_src: &str) -> ShaderHandles {
        let shader = self.device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(shader_src.into()),
            }
        );

        let pipeline = self.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &shader,
                entry_point: "main",
            }
        );
        
        ShaderHandles { shader, pipeline: PipelineHandle::Compute(pipeline) }
    }
    
    fn setup_buffer_handles<T>(&self, buffer: &T) -> BufferHandles
        where T: ?Sized 
    {
        let size = std::mem::size_of_val(&buffer) as u64;
        
        let staging_buffer = self.device.create_buffer(
            &wgpu::BufferDescriptor {
                label: None,
                size,
                usage: wgpu::BufferUsages::MAP_READ 
                | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }
        );
        
        let storage_buffer = self.device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("storage buffer"),
                size,
                usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }
        );
        
        BufferHandles {
            staging_buffer,
            storage_buffer,
        }
    }
}