use super::*;


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
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
        Vertex { position: [0.0, 0.5, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.4330127, -0.25, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [-0.4330127, -0.25, 0.0], color: [0.5, 0.0, 0.5] },
    ],
    indices: &[
        0, 2, 1,
    ]
};

pub const SQUARE: &Shape = &Shape {
    vertices: &[
        Vertex { position: [-0.5, 0.5, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.5, 0.5, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.5, -0.5, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [-0.5, -0.5, 0.0], color: [0.5, 0.0, 0.5] },
    ],
    indices: &[
        0, 3, 1,
        1, 3, 2,
    ]
};


pub const PENTAGON: &Shape = &Shape {
    vertices: &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },
    ],
    indices: &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ]
};

pub const HEXAGON: &Shape = &Shape {
    vertices: &[
        Vertex { position: [0.0, 0.0, 0.0], color: [0.5, 0.0, 0.5] },

        Vertex { position: [0.0000000, 0.5, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.4330127, 0.25, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.4330127, -0.25, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [0.0000000, -0.5, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [-0.4330127, -0.25, 0.0], color: [0.5, 0.0, 0.5] },
        Vertex { position: [-0.4330127, 0.25, 0.0], color: [0.5, 0.0, 0.5] },
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
