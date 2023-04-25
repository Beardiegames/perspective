use super::*;


pub struct ComputeSettings<'a, T>
where T: bytemuck::Pod + Clone
{
    pub label: &'a str, 
    pub group_index: u32,// represented within shader as @binding
    pub binding_index: u32,// represented within shader as @binding

    pub start_data: Vec<T>,
    pub shader_src: &'a str, // string slice representation of the actual shader code
    pub entry_point: &'a str, // name of the entry funcion/methode, called on update
}

pub struct ComputeProcessor //where T: ComputeData 
{
    pub compute_pipe: ComputePipeHandle, 
    pub bind_group: BindgroupHandle, 
    pub buffers: BufferHandle,
}

impl ComputeProcessor //where T: ComputeData 
{
    pub fn new<T>(core: &mut WgpuCore, settings: &ComputeSettings<T>) -> ComputeProcessor
    where T: bytemuck::Pod + Clone
    {
        //let data_clone = data.clone();
        let compute_pipe = ComputePipeHandle::new(
            core, 
            &format!("{}_compute-pipeline", settings.label),
            settings.shader_src, 
            settings.entry_point
        );
        let buffers = BufferHandle::new(
            core, 
            &format!("{}_compute-buffer", settings.label),
            settings.start_data.clone()
        );
        let bind_group = BindgroupHandle::new(
            core, 
            &BindGroupSettings {
                label: "compute_group", 
                group_index: settings.group_index,
                binding_index: settings.binding_index,

                pipeline: &compute_pipe, 
                resource_buffer: &buffers
            }
        );

        ComputeProcessor { compute_pipe, bind_group, buffers }
    }

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

        buffer_slice.map_async(wgpu::MapMode::Read, |_r|{});
        buffer_slice
    }

    pub fn read_results_and_drop<F, T>(&self, buffer_slice: wgpu::BufferSlice, map_bytes: F) -> Vec<T> 
    where   F: FnMut(&[u8]) -> T,
            T: bytemuck::Pod
    {
        let data = buffer_slice.get_mapped_range();

        let result: Vec<T> = data
            .chunks_exact(self.buffers.chunk_size)
            .map(map_bytes)
            .collect();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        self.buffers.staging.unmap();
        result
    }
}