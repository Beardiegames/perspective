
use super::*;
use wgpu::{util::DeviceExt, Surface};


impl WgpuCore {

    pub fn new(compatible_surface: Option<&Surface>) -> anyhow::Result<Self> {

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
                compatible_surface,
            })
            .block_on()
            .ok_or(anyhow::anyhow!("Couldn't create the adapter"))?;
            
        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .block_on()?;
        
        Ok( WgpuCore {
            instance,
            adapter,
            device,
            queue,

            bindgroup_count: 0,
        })
    }
    
    #[allow(dead_code)]
    /// @label: tag name for our shader module descriptor 
    /// @shader_src: actual shader code as a string
    /// @entry_point: name of the entry function within the shader code
    /// 
    pub fn setup_compute_pipeline(&self, label: &str, shader_src: &str, entry_point: &str) -> ShaderHandle {
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
                entry_point,
            }
        );
        
        ShaderHandle { shader, pipeline: PipelineHandle::Compute(pipeline) }
    }

    #[allow(dead_code)]
    /// @label: tag name for our buffer descriptor 
    /// @data: buffer data we want to share with our shader
    /// 
    pub fn setup_buffer_handles<T>(&self, label: &str, data: &[T]) -> BufferHandle
        where T: Sized + bytemuck::Pod
    {

        // determine memory size for our data
        let slice_size = data.len() * std::mem::size_of::<T>();
        let size = slice_size as wgpu::BufferAddress;
        
        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}_staging", label)),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Instantiates buffer with data (`numbers`).
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a shader).
        //   The destination of a copy.
        //   The source of a copy.
        let storage_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}_storage", label)),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        BufferHandle {
            staging: staging_buffer,
            storage: storage_buffer,
        }
    }


    /// A bind group defines how buffers are accessed by shaders.
    /// binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).
    /// 
    pub fn setup_bind_group(&mut self, label: &str, shader: &ShaderHandle, buffer: &BufferHandle) -> BindgroupHandle {

        let pipeline = match &shader.pipeline {
            PipelineHandle::Compute(p) => p,
        };

        let set_idx = self.bindgroup_count;
        let bond_idx = 0;

        let layout = pipeline.get_bind_group_layout(set_idx);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: bond_idx,
                resource: buffer.storage.as_entire_binding(),
            }],
        });

        self.bindgroup_count += 1;

        BindgroupHandle {
            bind_group,
            layout,
            set_idx,
            bond_idx
        }
    }
}