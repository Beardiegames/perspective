use crate::WgpuBinding;


pub struct Material {
    pub shader: wgpu::ShaderModule,
    pub bindings: Vec<WgpuBinding>,
    pub layout: wgpu::PipelineLayout,
    pub pipeline: wgpu::RenderPipeline,
}

pub struct MaterialBuilder<'a> {
    shader: wgpu::ShaderModule,

    bindings: Vec<WgpuBinding>,
    vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,

    vertex_entry: Option<&'static str>,
    fragment_entry: Option<&'static str>,
    tex_format: wgpu::TextureFormat,
    tex_blend: wgpu::BlendState,
}

impl<'a> MaterialBuilder<'a> {

    pub fn new(device: &wgpu::Device, shader_src: &str) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("Shader Module")),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });
        MaterialBuilder {
            shader,
            bindings: Vec::new(),
            vertex_buffers: Vec::new(),
            vertex_entry: None,
            fragment_entry: None,
            tex_format: wgpu::TextureFormat::Rgba32Float,
            tex_blend: wgpu::BlendState::ALPHA_BLENDING,
        }
    }

    pub fn add_binding(mut self, binding: WgpuBinding) -> Self {
        self.bindings.push(binding);
        self
    }
    pub fn add_vertex_buffer(mut self, vertex_buffer_layout: wgpu::VertexBufferLayout<'a>) -> Self {
        self.vertex_buffers.push(vertex_buffer_layout);
        self
    }

    pub fn set_vertex_entry_point(mut self, entry_point: &'static str) -> Self {
        self.vertex_entry = Some(entry_point);
        self
    }
    pub fn set_fragment_entry_point(mut self, entry_point: &'static str) -> Self {
        self.fragment_entry = Some(entry_point);
        self
    }
    pub fn set_texture_format(mut self, texture_format: wgpu::TextureFormat) -> Self {
        self.tex_format = texture_format;
        self
    }
    pub fn set_texture_blendstate(mut self, blend_state: wgpu::BlendState) -> Self {
        self.tex_blend = blend_state;
        self
    }

    pub fn build(self, device: &wgpu::Device) -> Material {
        let mut bind_group_layouts = Vec::<&wgpu::BindGroupLayout>::new();
        for binding in &self.bindings {
            bind_group_layouts.push(&binding.layout);
        }

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("Pipeline Layout")),
            bind_group_layouts: &bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("Render Pipeline")),

            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &self.shader,
                entry_point: match self.vertex_entry { Some(s) => s, None => "vert" },
                buffers: self.vertex_buffers.as_slice(), //&[Vertex::desc(), InstanceRaw::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: match self.fragment_entry { Some(s) => s, None => "frag" },
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.tex_format,
                    blend: Some(self.tex_blend),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Material {
            shader: self.shader,
            bindings: self.bindings,
            layout,
            pipeline,
        }
    }
}
