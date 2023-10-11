use super::*;
use wgpu::{util::DeviceExt};


pub struct ComputeSettings<'a, T>
where T: bytemuck::Pod + Clone
{
    pub label: &'a str, 
    pub data_set: Vec<T>,
    pub shader_src: &'a str, // string slice representation of the actual shader code
    pub entry_point: &'a str, // name of the entry funcion/methode, called on update
}

pub struct ComputeProcessor //where T: ComputeData 
{
    // pipeline
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::ComputePipeline,

    // buffers
    pub buffer_cellcount: u32,
    pub buffer_size: u64,
    pub buffer_chunksize: usize,
    pub staging_buffer: wgpu::Buffer,
    pub storage_buffer: wgpu::Buffer,

    // bind group
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group_index: u32,
    pub binding_index: u32,
}

impl ComputeProcessor //where T: ComputeData 
{
    pub fn new<T>(core: &mut WgpuCore, settings: &ComputeSettings<T>) -> ComputeProcessor
    where T: bytemuck::Pod + Clone
    {
        let shader = core.device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some(settings.label),
                source: wgpu::ShaderSource::Wgsl(settings.shader_src.into()),
            }
        );

        let pipeline = core.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &shader,
                entry_point: settings.entry_point,
            }
        );

        // determine memory size for our data
        let buffer_cellcount = settings.data_set.len();
        let buffer_chunksize = std::mem::size_of::<T>();
        let slice_size = buffer_cellcount * buffer_chunksize;
        let buffer_size = slice_size as wgpu::BufferAddress;
        
        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let staging_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}_stagingbuffer", settings.label)),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Instantiates buffer with data (`numbers`).
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a shader).
        //   The destination of a copy.
        //   The source of a copy.
        let storage_buffer = core.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}_storagebuffer", settings.label)),
            contents: bytemuck::cast_slice(settings.data_set.as_slice()),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let bind_group_index = 0;
        let binding_index = 0;

        let layout = pipeline.get_bind_group_layout(bind_group_index);
        let bind_group = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(settings.label),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: binding_index,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        ComputeProcessor { 
            shader,
            pipeline,
        
            // buffers
            buffer_cellcount: buffer_cellcount as u32,
            buffer_size,
            buffer_chunksize,
            staging_buffer,
            storage_buffer,
        
            // bind group
            bind_group,
            layout,
            bind_group_index,
            binding_index,
        }
    }

    pub fn quick_inject_passes(&self, encoder: &mut wgpu::CommandEncoder) {
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(self.bind_group_index, &self.bind_group, &[]);
            //cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch_workgroups(self.buffer_cellcount, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }
        encoder.copy_buffer_to_buffer(
            &self.storage_buffer, 
            0, 
            &self.staging_buffer, 
            0, 
            self.buffer_size
        );
    }

    pub fn slice_staging_buffer(&self) -> wgpu::BufferSlice {
        let buffer_slice = self.staging_buffer.slice(..);

        buffer_slice.map_async(wgpu::MapMode::Read, |_r|{});
        buffer_slice
    }

    pub fn read_results_and_drop<F, T>(&self, buffer_slice: wgpu::BufferSlice, map_bytes: F) -> Vec<T> 
    where   F: FnMut(&[u8]) -> T,
            T: bytemuck::Pod
    {
        let data = buffer_slice.get_mapped_range();

        let result: Vec<T> = data
            .chunks_exact(self.buffer_chunksize)
            .map(map_bytes)
            .collect();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        self.staging_buffer.unmap();
        result
    }
}