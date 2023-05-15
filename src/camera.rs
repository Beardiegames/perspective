use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use crate::*;


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct GpuCameraHandle {
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub binding: wgpu::BindGroup,
}

impl GpuCameraHandle {
    pub fn new(device: &wgpu::Device, camera_uniform: CameraUniform) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        
        let binding = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        
        GpuCameraHandle { buffer, layout, binding }
    }
}

pub type ProjectionSize = f32;
pub type ProjectionFOV = f32;

pub enum CameraProjection {
    Orthographic(ProjectionSize),
    Perspective(ProjectionFOV)
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
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
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
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            eye: cgmath::Point3{x: 0.0, y: 1.0, z: 2.0},
            target: cgmath::Point3{x: 0.0, y: 0.0, z: 0.0},
            up: cgmath::Vector3::unit_y(),
            aspect: 16.0/9.0,
            projection: CameraProjection::Perspective(45.0),
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

unsafe impl Pod for CameraUniform {}
unsafe impl Zeroable for CameraUniform {}

