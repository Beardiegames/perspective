//use super::*;


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub col: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

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
        Vertex { pos: [0.0, 0.5, 0.0],          col: [0.5, 0.0, 0.5], uv: [0.5, 0.0] },
        Vertex { pos: [0.4330127, -0.25, 0.0],  col: [0.5, 0.0, 0.5], uv: [1.0, 1.0] },
        Vertex { pos: [-0.4330127, -0.25, 0.0], col: [0.5, 0.0, 0.5], uv: [0.0, 1.0] },
    ],
    indices: &[
        0, 2, 1,
    ]
};

pub const SQUARE: &Shape = &Shape {
    vertices: &[
        Vertex { pos: [-0.5, 0.5, 0.0],     col: [0.5, 0.0, 0.5], uv: [0.0, 0.0] },
        Vertex { pos: [0.5, 0.5, 0.0],      col: [0.5, 0.0, 0.5], uv: [1.0, 0.0] },
        Vertex { pos: [0.5, -0.5, 0.0],     col: [0.5, 0.0, 0.5], uv: [1.0, 1.0] },
        Vertex { pos: [-0.5, -0.5, 0.0],    col: [0.5, 0.0, 0.5], uv: [0.0, 1.0] },
    ],
    indices: &[
        0, 3, 1,
        1, 3, 2,
    ]
};


pub const PENTAGON: &Shape = &Shape {
    vertices: &[
        Vertex { pos: [-0.0868241, 0.49240386, 0.0],    col: [0.5, 0.0, 0.5], uv: [0.4131759, 0.99240386] },
        Vertex { pos: [-0.49513406, 0.06958647, 0.0],   col: [0.5, 0.0, 0.5], uv: [0.0048659444, 0.56958647] },
        Vertex { pos: [-0.21918549, -0.44939706, 0.0],  col: [0.5, 0.0, 0.5], uv: [0.28081453, 0.05060294] },
        Vertex { pos: [0.35966998, -0.3473291, 0.0],    col: [0.5, 0.0, 0.5], uv: [0.85967, 0.1526709] },
        Vertex { pos: [0.44147372, 0.2347359, 0.0],     col: [0.5, 0.0, 0.5], uv: [0.9414737, 0.7347359] },
    ],
    indices: &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ]
};

pub const HEXAGON: &Shape = &Shape {
    vertices: &[
        Vertex { pos: [0.0, 0.0, 0.0],  col: [0.5, 0.5, 0.5],   uv: [0.5, 0.5] },

        Vertex { pos: [0.0000000, 0.5, 0.0],    col: [1.0, 0.0, 0.0],   uv: [0.5, 0.0] },
        Vertex { pos: [0.4330127, 0.25, 0.0],   col: [0.75, 0.75, 0.0], uv: [1.0, 0.25] },
        Vertex { pos: [0.4330127, -0.25, 0.0],  col: [0.0, 1.0, 0.0],   uv: [1.0, 0.75] },
        Vertex { pos: [0.0000000, -0.5, 0.0],   col: [0.0, 0.75, 0.75], uv: [0.5, 1.0] },
        Vertex { pos: [-0.4330127, -0.25, 0.0], col: [0.0, 0.0, 1.0],   uv: [0.0, 0.75] },
        Vertex { pos: [-0.4330127, 0.25, 0.0],  col: [0.75, 0.0, 0.75], uv: [0.0, 0.25] },
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
