use super::*;
use wgpu::{util::DeviceExt, Surface};


pub struct BufferHandle { //<T> {
    //pub data: Vec<T>,
    pub cell_count: u32,
    pub buffer_size: u64,
    pub chunk_size: usize,

    pub staging: wgpu::Buffer,
    pub storage: wgpu::Buffer,
}

//impl<T> BufferHandle<T> {
impl BufferHandle {

    #[allow(dead_code)]
    /// @label: tag name for our buffer descriptor 
    /// @data: buffer data we want to share with our shader
    /// 
    pub fn new<T>(core: &WgpuCore, label: &str, data: Vec<T>) -> Self
        where T: Sized + bytemuck::Pod
    {

        // determine memory size for our data
        let cell_count = data.len();
        let chunk_size = std::mem::size_of::<T>();
        let slice_size = cell_count * chunk_size;
        let buffer_size = slice_size as wgpu::BufferAddress;
        
        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let staging_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}_staging", label)),
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
            label: Some(&format!("{}_storage", label)),
            contents: bytemuck::cast_slice(data.as_slice()),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        BufferHandle{
            //data,
            cell_count: cell_count  as u32,
            buffer_size,
            chunk_size,
            staging: staging_buffer,
            storage: storage_buffer,
        }
    }
}