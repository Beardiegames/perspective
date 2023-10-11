use super::*;
use std::future::*;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use wgpu::{ 
	BufferAsyncError 
};

// #[derive(Clone)]
// pub enum ComputeError {
	// PoisonedMutexGuard,
	// BufferError(BufferAsyncError),
// }
pub trait FromBytes {
	fn from_bytes(b: &[u8]) -> Self;
}

pub type ComputeResult<'a, T> = Result<&'a Vec<T>, BufferAsyncError>;

pub struct ComputeProcess<'a, T>(Arc<Mutex<Option<ComputeResult<'a, T>>>>);

impl<'a, T> ComputeProcess<'a, T> {

	fn new() -> ComputeProcess<'a, T> {
		ComputeProcess(Arc::new(Mutex::new(None)))
	}

	fn finish(&mut self, data: &'a Vec<T>) {
		if let Ok(state) = self.0.lock() {
			(*state) = Some(Ok(data));
		}
		else {
			panic!("ComputeProcess.finish() :: MutexGuard Poisoned!");
		}
	}

	fn throw(&mut self, e: BufferAsyncError) {
		if let Ok(state) = self.0.lock() {
			(*state) = Some(Err(e));
		}
		else {
			panic!("ComputeProcess.throw() :: MutexGuard Poisoned!");
		}
	}
	
	pub fn state(&self) -> std::task::Poll< ComputeResult<'a, T>>  {
		if let Ok(state) = self.0.lock() {
				
			match *state {
				Some(r) => {
					match r {
						Ok(data) => Poll::Ready(Ok(&data)),
						Err(e) => Poll::Ready(Err(e.clone())),
					}
				},
				None => Poll::Pending,
			}
		} 
		else {
			panic!("ComputeProcess.state() :: MutexGuard Poisoned!")
		}
	}
}

impl<'a, T> Clone for ComputeProcess<'a, T> {
	fn clone(&self) -> Self {
		ComputeProcess(Arc::clone(&self.0))
	}
}

impl<'a, T> Future for ComputeProcess<'a, T> {
	type Output = ComputeResult<'a, T>;
	
	fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
		if let Ok(state) = self.0.lock() {
		
			match *state {
				Some(r) => {
					match r {
						Ok(data) => Poll::Ready(Ok(&data)),
						Err(e) => Poll::Ready(Err(e.clone())),
					}
				},
				None => Poll::Pending,
			}
		} 
		else {
			panic!("ComputeProcess.poll() :: MutexGuard Poisoned!")
		}
	}
}

impl WgpuCore {

    pub fn execute_compute_passes<'a, T>(&self, 
        compute_pipe: &ComputePipeHandle, 
        bind_group: &BindgroupHandle, 
        buffers: &BufferHandle<'a, T>
    ) -> ComputeProcess<'a, T> 
		where T: FromBytes + std::marker::Sync
    {
    
        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipe.pipeline);
            cpass.set_bind_group(*bind_group.set_idx(), &bind_group.bind_group, &[]);
            cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch_workgroups(buffers.data.len() as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(&buffers.storage, 0, &buffers.staging, 0, buffers.size);

        // Submits command encoder for processing
        self.queue.submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = buffers.staging.slice(..);
        // Gets the future representing when `staging_buffer` can be read from

    	let compute_process: ComputeProcess<'a, T> = ComputeProcess::new();
    	let process_handle = compute_process.clone();
    	    
        let buffer_future = buffer_slice.map_async(
        	wgpu::MapMode::Read, 
        	|compute_result| {
       			match compute_result {
		    		Ok(()) => {
		    			 // Gets contents of buffer
			            let data = buffer_slice.get_mapped_range();
			            // Since contents are got in bytes, this converts these bytes back to u32
			            let result: Vec<T> = data
			                .chunks_exact(4)
			                .map(|b| T::from_bytes(b.try_into().unwrap()))
			                //.map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
			                .collect();

			            // With the current interface, we have to make sure all mapped views are
			            // dropped before we unmap the buffer.
			            drop(data);


			            // Returns data from buffer
			            process_handle.finish(&result);
		    		},
		    		Err(e) => {
		    			//panic!("failed to run compute on gpu!");
		    			process_handle.throw(e);
		    		},
				}
        	}

     		
            
        );

        compute_process
    }

    pub fn unwrap_compute_result<'a, T>(&self, buffers: &BufferHandle<'a, T>) {


        buffers.staging.unmap();// Unmaps buffer from memory
		                        // If you are familiar with C++ these 2 lines can be thought of similarly to:
		                        //   delete myPointer;
		                        //   myPointer = NULL;
		                        // It effectively frees the memory
    }
}
