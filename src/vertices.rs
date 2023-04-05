use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

pub fn new_vertex_buffer(device: &wgpu::Device) -> (wgpu::Buffer, u32) {
    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );
    let num_vertices = VERTICES.len() as u32;
    
    (vertex_buffer, num_vertices)
}
        
pub fn new_index_buffer(device: &wgpu::Device) -> (wgpu::Buffer, u32) {
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    );
    let num_indices = INDICES.len() as u32;
    
    (index_buffer, num_indices)
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ]
        }
    }
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}


pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 0.0], },
    Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 1.0], },
    Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 1.0], },
    
    // Changed
    // Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    // Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    // Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    // Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    // Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];


pub const INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3,
    // 0, 1, 4,
    // 1, 2, 4,
    // 2, 3, 4,
];

 

 

