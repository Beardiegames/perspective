use wgpu::{ShaderModel, Device};
use crate::{WgpuDataBinding, WgpuTextureBinding};


pub struct Material {
    pub shader: wgpu::ShaderModule,
    pub texture_binding: WgpuTextureBinding,
    pub data_bindings: Vec<WgpuDataBinding>,
    pub layout: wgpu::PipelineLayout,
}


impl Material {
    pub fn new(
        device: Device,
        shader_src: &str,
        texture_binding: WgpuTextureBinding,
        data_bindings: Vec<WgpuDataBinding>,

    ) -> Material {

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("Shader Module")),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        let mut bind_group_layouts = Vec::<&wgpu::BindGroupLayout>::new();
        for binding in &data_bindings {
            bind_group_layouts.push(&binding.layout);
        }

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("Pipeline Layout")),
            bind_group_layouts: &bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        Material {
            shader,
            texture_binding,
            data_bindings,
            layout,
        }
    }
}