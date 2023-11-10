//use super::*;
use wgpu::{util::DeviceExt, Buffer};


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub col: [f32; 3],
    pub uv_map: [f32; 2],
    pub uv_scale: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 5] =
        wgpu::vertex_attr_array![
            0 => Float32x3, 
            1 => Float32x3, 
            2 => Float32x2,
            3 => Float32x2, 
            4 => Float32x3, 
        ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Shape {

    pub fn setup_wgpu_buffers(&self, device: &wgpu::Device,) -> (Buffer, Buffer) {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(self.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(self.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        (
            vertex_buffer,
            index_buffer
        )
    }
}

// pub fn create_triangle(uv_scale: [f32; 2]) -> Shape {
//     Shape {
//         vertices: vec![
//             Vertex { pos: [0.0, 0.5, 0.0],          col: [0.5, 0.0, 0.5], uv_map: [0.5, 0.0], uv_scale, normal: [0.5, 0.0, 0.0] },
//             Vertex { pos: [0.4330127, -0.25, 0.0],  col: [0.5, 0.0, 0.5], uv_map: [1.0, 1.0], uv_scale, normal: [1.0, 1.0, 0.0] },
//             Vertex { pos: [-0.4330127, -0.25, 0.0], col: [0.5, 0.0, 0.5], uv_map: [0.0, 1.0], uv_scale, normal: [0.0, 1.0, 0.0] },
//         ],
//         indices: vec![
//             0, 2, 1,
//         ]
//     }
// }

pub fn create_square(uv_scale: [f32; 2]) -> Shape {
    Shape {
        vertices: vec![
            Vertex { pos: [-0.5, 0.5, 0.0],     col: [0.5, 0.0, 0.5], uv_map: [0.0, 0.0], uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [0.5, 0.5, 0.0],      col: [0.5, 0.0, 0.5], uv_map: [1.0, 0.0], uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [0.5, -0.5, 0.0],     col: [0.5, 0.0, 0.5], uv_map: [1.0, 1.0], uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [-0.5, -0.5, 0.0],    col: [0.5, 0.0, 0.5], uv_map: [0.0, 1.0], uv_scale, normal: [0.0, 0.0, 1.0] },
        ],
        indices: vec![
            0, 3, 1,
            1, 3, 2,
        ]
    }
}

// pub fn create_pentagon(uv_scale: [f32; 2]) -> Shape {
//     Shape {
//         vertices: vec![
//             Vertex { pos: [-0.0868241, 0.49240386, 0.0],    col: [0.5, 0.0, 0.5], uv_map: [0.4131759, 0.99240386],      uv_scale, normal: [0.4131759, 0.99240386, 0.0] },
//             Vertex { pos: [-0.49513406, 0.06958647, 0.0],   col: [0.5, 0.0, 0.5], uv_map: [0.0048659444, 0.56958647],   uv_scale, normal: [0.0048659444, 0.56958647, 0.0] },
//             Vertex { pos: [-0.21918549, -0.44939706, 0.0],  col: [0.5, 0.0, 0.5], uv_map: [0.28081453, 0.05060294],     uv_scale, normal: [0.28081453, 0.05060294, 0.0] },
//             Vertex { pos: [0.35966998, -0.3473291, 0.0],    col: [0.5, 0.0, 0.5], uv_map: [0.85967, 0.1526709],         uv_scale, normal: [0.85967, 0.1526709, 0.0] },
//             Vertex { pos: [0.44147372, 0.2347359, 0.0],     col: [0.5, 0.0, 0.5], uv_map: [0.9414737, 0.7347359],       uv_scale, normal: [0.9414737, 0.7347359, 0.0] },
//         ],
//         indices: vec![
//             0, 1, 4,
//             1, 2, 4,
//             2, 3, 4,
//         ]
//     }
// }

#[allow(dead_code)]
pub fn create_hexagon(uv_scale: [f32; 2]) -> Shape {
    Shape {
        vertices: vec![
            Vertex { pos: [0.0, 0.0, 0.0],  col: [0.5, 0.5, 0.5],   uv_map: [0.5, 0.5], uv_scale, normal: [0.0, 0.0, 0.1] },

            Vertex { pos: [0.0000000, 0.5, 0.0],    col: [1.00, 0.00, 0.00], uv_map: [0.5, 0.00],       uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [0.4330127, 0.25, 0.0],   col: [0.75, 0.75, 0.00], uv_map: [0.9330127, 0.25], uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [0.4330127, -0.25, 0.0],  col: [0.00, 1.00, 0.00], uv_map: [0.9330127, 0.75], uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [0.0000000, -0.5, 0.0],   col: [0.00, 0.75, 0.75], uv_map: [0.5, 1.00],       uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [-0.4330127, -0.25, 0.0], col: [0.00, 0.00, 1.00], uv_map: [0.0669873, 0.75], uv_scale, normal: [0.0, 0.0, 1.0] },
            Vertex { pos: [-0.4330127, 0.25, 0.0],  col: [0.75, 0.00, 0.75], uv_map: [0.0669873, 0.25], uv_scale, normal: [0.0, 0.0, 1.0] },
        ],
        indices: vec![
            0, 2, 1,
            0, 3, 2,
            0, 4, 3,
            0, 5, 4,
            0, 6, 5,
            0, 1, 6,
        ]
    }
}
