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


pub fn create_camera_buffer(canvas: &Canvas, camera_uniform: CameraUniform) -> wgpu::Buffer {
    canvas.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    )
}

pub fn create_camera_bindgroup(canvas: &Canvas, camera_buffer: &wgpu::Buffer) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let camera_bind_group_layout = canvas.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
    
    let camera_bind_group = canvas.device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }
        ],
        label: Some("camera_bind_group"),
    });
    
    (camera_bind_group_layout, camera_bind_group)
}

pub enum Projection {
    Otrho { size: f32 },
    Perspective { fov: f32 },
}


pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub znear: f32,
    pub zfar: f32,
    pub projection: Projection,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        
        let proj = match self.projection {
            Projection::Perspective { fov } => {
                cgmath::perspective(cgmath::Deg(fov), self.aspect, self.znear, self.zfar)
            },
            Projection::Otrho { size } => {
                let (h, w) = (size, size * self.aspect); //self.fovy * self.aspect;
                cgmath::ortho(-w, w, -h, h, self.znear, self.zfar)
            },
        };

        return OPENGL_TO_WGPU_MATRIX * proj * view;
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

