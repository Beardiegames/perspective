use bytemuck::{Pod, Zeroable};
use crate::WgpuDataBinding;
use crate::WgpuCore;
use crate::bindings;


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


pub type ProjectionSize = f32;
pub type ProjectionFOV = f32;

#[derive(Clone)]
pub enum CameraProjection {
    Orthographic(ProjectionSize),
    Perspective(ProjectionFOV)
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable, Default)]
pub struct CameraUniform {
    projection_matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            projection_matrix: cgmath::Matrix4::identity().into(),
        }
    }
}

#[derive(Clone)]
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
            zfar: 1000.0,
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

    pub uniform: CameraUniform,
    pub binding: WgpuDataBinding,
}

impl Camera {
    pub fn new(device: &wgpu::Device, layout:&wgpu::BindGroupLayout, setup: &CameraSetup) -> Self {
        let uniform = CameraUniform::new();
        let binding = bindings::create_camera_binding(device, layout, uniform);

        Camera {
            eye: setup.eye,
            target: setup.target,
            up: setup.up,
            aspect: setup.aspect,
            projection: setup.projection.clone(),
            znear: setup.znear,
            zfar: setup.zfar,

            uniform,
            binding
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

    pub fn buffer_update(&mut self, gx: &WgpuCore) {
        self.uniform.projection_matrix = self.build_view_map().into();
        
        gx.queue.write_buffer(
            &self.binding.buffers[0], 
            0, 
            bytemuck::cast_slice(&[self.uniform])
        );
    }
}




