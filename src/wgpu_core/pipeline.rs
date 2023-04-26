use super::*;


pub trait PipelineHandle {
    fn get_bind_group_layout(&self, set_idx: u32) -> BindGroupLayout;
}

pub struct RenderPipeHandle {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
}

pub struct ComputePipeHandle {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::ComputePipeline,
}

impl PipelineHandle for RenderPipeHandle {
    fn get_bind_group_layout(&self, idx: u32) -> BindGroupLayout {
        self.pipeline.get_bind_group_layout(idx) 
    }
}

impl PipelineHandle for ComputePipeHandle {
    fn get_bind_group_layout(&self, idx: u32) -> BindGroupLayout {
        self.pipeline.get_bind_group_layout(idx) 
    }
}

impl RenderPipeHandle {

    #[allow(dead_code)]
    /// @label: tag name for our shader module descriptor 
    /// @shader_src: actual shader code as a string
    /// @entry_point: name of the entry function within the shader code
    /// 
    pub fn new(
        core: &WgpuCore, 
        label: &str, 
        shader_src: &str, 
        vertex_entry_point: &str, 
        fragment_entry_point: &str, 
    ) -> RenderPipeHandle 
    {
        let texture_format = core.canvas.as_ref()
            .expect("Canvas not available! Try passing WindowSettings when creating a new WgpuCore.")
            .config.format;

        let shader = core.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{}_shader", label)),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        let layout = core.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{}_layout", label)),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = core.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{}_pipeline", label)),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: vertex_entry_point,
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: fragment_entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        RenderPipeHandle { shader, pipeline }
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