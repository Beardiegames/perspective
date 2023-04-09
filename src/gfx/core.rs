
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


    pub fn setup_compute_command_encoder<'a, T>(&self, compute_pipe: &ComputePipeHandle, bind_group: &BindgroupHandle, buffers: &BufferHandle<'a, T>) {
        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipe.pipeline);
            cpass.set_bind_group(0, &bind_group.bind_group, &[]);
            //cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch_workgroups(buffers.data.len() as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(&buffers.storage, 0, &buffers.staging, 0, buffers.size);

        // Submits command encoder for processing
        self.queue.submit(Some(encoder.finish()));
    }
}