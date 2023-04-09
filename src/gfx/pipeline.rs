use super::*;


impl WgpuCore {

    #[allow(dead_code)]
    /// @label: tag name for our shader module descriptor 
    /// @shader_src: actual shader code as a string
    /// @entry_point: name of the entry function within the shader code
    /// 
    pub fn setup_compute_pipeline(&self, label: &str, shader_src: &str, entry_point: &str) -> ComputePipeHandle {
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
        
        ComputePipeHandle { shader, pipeline }
    }

}