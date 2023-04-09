use super::*;
use wgpu::{util::DeviceExt, Surface};


impl WgpuCore {

    #[allow(dead_code)]
    /// @label: tag name for our buffer descriptor 
    /// @data: buffer data we want to share with our shader
    /// 
    pub fn create_buffer_handles<'a, T>(&self, label: &str, data: &'a [T]) -> BufferHandle<'a, T>
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

        BufferHandle{
            data,
            size,
            staging: staging_buffer,
            storage: storage_buffer,
        }
    }

}