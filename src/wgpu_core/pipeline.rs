use super::*;


pub trait PipelineHandle {
    fn get_bind_group_layout(&self, set_idx: u32) -> BindGroupLayout;
}

pub struct ComputePipeHandle {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::ComputePipeline,
}

impl PipelineHandle for ComputePipeHandle {
    fn get_bind_group_layout(&self, idx: u32) -> BindGroupLayout {
        self.pipeline.get_bind_group_layout(idx) 
    }
}

impl ComputePipeHandle {

    #[allow(dead_code)]
    /// @label: tag name for our shader module descriptor 
    /// @shader_src: actual shader code as a string
    /// @entry_point: name of the entry function within the shader code
    /// 
    pub fn new(core: &WgpuCore, label: &str, shader_src: &str, entry_point: &str) -> ComputePipeHandle {
        let shader = core.device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(shader_src.into()),
            }
        );

        let pipeline = core.device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &shader,
                entry_point,
            }
        );
        
        ComputePipeHandle { shader, pipeline }
    }

}