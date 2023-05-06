//use super::*;


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub col: [f32; 3],
    pub uv: [f32; 2],
    pub tile: i32,
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Sint32];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Shape<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
}

pub const TRIANGLE: &Shape = &Shape {
    vertices: &[
        Vertex { pos: [0.0, 0.5, 0.0],          col: [0.5, 0.0, 0.5], uv: [0.5, 0.0], tile: 0 },
        Vertex { pos: [0.4330127, -0.25, 0.0],  col: [0.5, 0.0, 0.5], uv: [1.0, 1.0], tile: 0 },
        Vertex { pos: [-0.4330127, -0.25, 0.0], col: [0.5, 0.0, 0.5], uv: [0.0, 1.0], tile: 0 },
    ],
    indices: &[
        0, 2, 1,
    ]
};

pub const SQUARE: &Shape = &Shape {
    vertices: &[
        Vertex { pos: [-0.5, 0.5, 0.0],     col: [0.5, 0.0, 0.5], uv: [0.0, 0.0], tile: 0 },
        Vertex { pos: [0.5, 0.5, 0.0],      col: [0.5, 0.0, 0.5], uv: [1.0, 0.0], tile: 0 },
        Vertex { pos: [0.5, -0.5, 0.0],     col: [0.5, 0.0, 0.5], uv: [1.0, 1.0], tile: 0 },
        Vertex { pos: [-0.5, -0.5, 0.0],    col: [0.5, 0.0, 0.5], uv: [0.0, 1.0], tile: 0 },
    ],
    indices: &[
        0, 3, 1,
        1, 3, 2,
    ]
};


pub const PENTAGON: &Shape = &Shape {
    vertices: &[
        Vertex { pos: [-0.0868241, 0.49240386, 0.0],    col: [0.5, 0.0, 0.5], uv: [0.4131759, 0.99240386],      tile: 0 },
        Vertex { pos: [-0.49513406, 0.06958647, 0.0],   col: [0.5, 0.0, 0.5], uv: [0.0048659444, 0.56958647],   tile: 0 },
        Vertex { pos: [-0.21918549, -0.44939706, 0.0],  col: [0.5, 0.0, 0.5], uv: [0.28081453, 0.05060294],     tile: 0 },
        Vertex { pos: [0.35966998, -0.3473291, 0.0],    col: [0.5, 0.0, 0.5], uv: [0.85967, 0.1526709],         tile: 0 },
        Vertex { pos: [0.44147372, 0.2347359, 0.0],     col: [0.5, 0.0, 0.5], uv: [0.9414737, 0.7347359],       tile: 0 },
    ],
    indices: &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ]
};

pub const HEXAGON: &Shape = &Shape {
    vertices: &[
        Vertex { pos: [0.0, 0.0, 0.0],  col: [0.5, 0.5, 0.5],   uv: [0.5, 0.5], tile: 0 },

        Vertex { pos: [0.0000000, 0.5, 0.0],    col: [1.00, 0.00, 0.00], uv: [0.5, 0.00],       tile: 0 },
        Vertex { pos: [0.4330127, 0.25, 0.0],   col: [0.75, 0.75, 0.00], uv: [0.9330127, 0.25], tile: 0 },
        Vertex { pos: [0.4330127, -0.25, 0.0],  col: [0.00, 1.00, 0.00], uv: [0.9330127, 0.75], tile: 0 },
        Vertex { pos: [0.0000000, -0.5, 0.0],   col: [0.00, 0.75, 0.75], uv: [0.5, 1.00],       tile: 0 },
        Vertex { pos: [-0.4330127, -0.25, 0.0], col: [0.00, 0.00, 1.00], uv: [0.0669873, 0.75], tile: 0 },
        Vertex { pos: [-0.4330127, 0.25, 0.0],  col: [0.75, 0.00, 0.75], uv: [0.0669873, 0.25], tile: 0 },
    ],
    indices: &[
        0, 2, 1,
        0, 3, 2,
        0, 4, 3,
        0, 5, 4,
        0, 6, 5,
        0, 1, 6,
    ]
};
