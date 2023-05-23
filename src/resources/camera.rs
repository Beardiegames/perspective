use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::WgpuCore;


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub type ProjectionSize = f32;
pub type ProjectionFOV = f32;

pub enum CameraProjection {
    Orthographic(ProjectionSize),
    Perspective(ProjectionFOV)
}


// #[repr(C)]
// #[derive(Debug, Copy, Clone, Pod, Zeroable)]
// pub struct ProjectionMap {
//     view_proj: [[f32; 4]; 4],
// }

// impl ProjectionMap {
//     pub fn new() -> Self {
//         use cgmath::SquareMatrix;
//         Self {
//             view_proj: cgmath::Matrix4::identity().into(),
//         }
//     }
// }


pub struct CameraSetup {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub projection: CameraProjection,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for CameraSetup {
    fn default() -> Self {
        CameraSetup {
            eye: cgmath::Point3{x: 0.0, y: 1.0, z: 2.0},
            target: cgmath::Point3{x: 0.0, y: 0.0, z: 0.0},
            up: cgmath::Vector3::unit_y(),
            aspect: 16.0/9.0,
            projection: CameraProjection::Perspective(80.0),
            znear: 0.1,
            zfar: 100.0,
        }
    }
}


pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub projection: CameraProjection,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new(setup: CameraSetup) -> Self {
        Camera {
            eye: setup.eye,
            target: setup.target,
            up: setup.up,
            aspect: setup.aspect,
            projection: setup.projection,
            znear: setup.znear,
            zfar: setup.zfar,
        }
    }

    pub fn build_view_map(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        let proj = match self.projection {
            CameraProjection::Orthographic(size) => {
                cgmath::ortho(0.0 - size * self.aspect, size * self.aspect, 0.0 - size, size, self.znear, self.zfar)
            },
            CameraProjection::Perspective(fov) => {
                cgmath::perspective(cgmath::Deg(fov), self.aspect, self.znear, self.zfar)
            },
        };

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    // pub fn update_projection_map(&mut self, gx: &WgpuCore) {
    //     self.map.view_proj = self.build_view_map().into();

    //     gx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.map]));
    // }
}




