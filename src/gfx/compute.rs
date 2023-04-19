use wgpu::BufferAsyncError;

use super::*;
use std::sync::{Arc, Mutex};

pub trait ComputeData: Clone + Sized + bytemuck::Pod + std::marker::Send + std::marker::Sync 
{
    fn from_bytes(b: &[u8]) -> Self;
}

pub struct ComputeProcessor<T> where T: ComputeData 
{
    pub compute_pipe: ComputePipeHandle, 
    pub bind_group: BindgroupHandle, 
    pub buffers: BufferHandle<T>,

    data_access: Arc<Mutex<Vec<T>>>,
}

impl<T> ComputeProcessor<T> where T: ComputeData 
{
    pub fn new(core: &mut WgpuCore, shader_str: &str, data: Vec<T>) -> ComputeProcessor<T> 
        where T: ComputeData
    {
        let data_clone = data.clone();
        let compute_pipe = ComputePipeHandle::new(core, "compute_pipe", shader_str, "main");
        let buffers = BufferHandle::new(core, "compute_buffer", data_clone);
        let bind_group = BindgroupHandle::new(core, "compute_group", &compute_pipe, &buffers);
        let data_access = Arc::new(Mutex::new(Vec::from(data)));

        ComputeProcessor { compute_pipe, bind_group, buffers, data_access }
    }

    // pub fn data_ref(&self) -> &[T] {
    //     &(*(*self.data_access).lock().unwrap()).as_slice()
    // }

    // pub fn data_mut(&mut self) -> &mut Vec<T> {
    //     &mut *(*self.data_access).lock().unwrap()
    // }

    pub fn clone_data_access(&self) -> Arc<Mutex<Vec<T>>> {
        Arc::clone(&self.data_access)
    }

    pub fn execute(&self, core: &WgpuCore) -> wgpu::BufferSlice
    {

        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder = core.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.compute_pipe.pipeline);
            cpass.set_bind_group(*self.bind_group.set_idx(), &self.bind_group.bind_group, &[]);
            cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch_workgroups(self.buffers.data.len() as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(&self.buffers.storage, 0, &self.buffers.staging, 0, self.buffers.size);

        // Submits command encoder for processing
        core.queue.submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = self.buffers.staging.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        
        //let data_clone = Arc::clone(&self.data_access);
        
        buffer_slice.map_async(wgpu::MapMode::Read, |r|{});
        buffer_slice
    }

    pub fn post_render(&self, buffer_slice: wgpu::BufferSlice) -> Vec<T> {

        let chunksize = std::mem::size_of::<T>();
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result: Vec<T> = data
            .chunks_exact(chunksize)
            .map(|b| T::from_bytes(b.try_into().unwrap()))
            //.map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
            .collect();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        
        self.buffers.staging.unmap();
            // Unmaps buffer from memory
            // If you are familiar with C++ these 2 lines can be thought of similarly to:
            //   delete myPointer;
            //   myPointer = NULL;
            // It effectively frees the memory

        result
    }
}