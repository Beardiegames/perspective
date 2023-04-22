use wgpu::BufferAsyncError;

use super::*;
use std::sync::{Arc, Mutex};


pub struct ComputeProcessor //where T: ComputeData 
{
    pub compute_pipe: ComputePipeHandle, 
    pub bind_group: BindgroupHandle, 
    pub buffers: BufferHandle,

    //data_access: Arc<Mutex<Vec<T>>>,
}

impl ComputeProcessor //where T: ComputeData 
{
    pub fn new<T>(core: &mut WgpuCore, shader_str: &str, data: Vec<T>) -> ComputeProcessor
    where T: bytemuck::Pod
    {
        //let data_clone = data.clone();
        let compute_pipe = ComputePipeHandle::new(core, "compute_pipe", shader_str, "main");
        let buffers = BufferHandle::new(core, "compute_buffer", data);
        let bind_group = BindgroupHandle::new(core, "compute_group", &compute_pipe, &buffers);
        //let data_access = Arc::new(Mutex::new(Vec::from(data)));

        ComputeProcessor { compute_pipe, bind_group, buffers }//, data_access }
    }

    // pub fn data_ref(&self) -> &[T] {
    //     &(*(*self.data_access).lock().unwrap()).as_slice()
    // }

    // pub fn data_mut(&mut self) -> &mut Vec<T> {
    //     &mut *(*self.data_access).lock().unwrap()
    // }

    // pub fn clone_data_access(&self) -> Arc<Mutex<Vec<T>>> {
    //     Arc::clone(&self.data_access)
    // }

    pub fn inject_passes(&self, encoder: &mut wgpu::CommandEncoder) {
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.compute_pipe.pipeline);
            cpass.set_bind_group(*self.bind_group.set_idx(), &self.bind_group.bind_group, &[]);
            cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch_workgroups(self.buffers.cell_count, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }
        encoder.copy_buffer_to_buffer(&self.buffers.storage, 0, &self.buffers.staging, 0, self.buffers.buffer_size);
    }

    pub fn slice_staging_buffer(&self) -> wgpu::BufferSlice {
        let buffer_slice = self.buffers.staging.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        
        //let data_clone = Arc::clone(&self.data_access);
        
        buffer_slice.map_async(wgpu::MapMode::Read, |r|{});
        buffer_slice
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
            cpass.dispatch_workgroups(self.buffers.cell_count, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(&self.buffers.storage, 0, &self.buffers.staging, 0, self.buffers.buffer_size);

        // Submits command encoder for processing
        core.queue.submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = self.buffers.staging.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        
        //let data_clone = Arc::clone(&self.data_access);
        
        buffer_slice.map_async(wgpu::MapMode::Read, |r|{});
        buffer_slice
    }

    pub fn read_results_and_drop<F, T>(&self, buffer_slice: wgpu::BufferSlice, map_bytes: F) -> Vec<T> 
    where   F: FnMut(&[u8]) -> T,
            T: bytemuck::Pod
    {
        //let chunksize = std::mem::size_of::<T>();
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result: Vec<T> = data
            .chunks_exact(self.buffers.chunk_size)
            .map(map_bytes)
            //.map(|b| T::from_bytes(b.try_into().unwrap()))
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