use crate::{WgpuGrapics, WgpuDataBinding, create_lights_binding};


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AmbientLight {
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLight {
    pub position: [f32; 4],
    pub color: [f32; 4],
    //_padding: u32, // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
}

pub struct PointLightSetup {
    pub position: cgmath::Vector3<f32>,
    pub color: [f32; 4],
    pub power: f32,
}

impl From<PointLightSetup> for PointLight {
    fn from(other: PointLightSetup) -> PointLight {
        PointLight {
            position: [
                other.position.x, 
                other.position.y, 
                other.position.z, 
                other.power
            ],
            color: other.color,
        }
    }
}

pub struct Light {
    pub ambient: AmbientLight,
    pub point_lights: Vec<PointLight>,
    pub binding: WgpuDataBinding,
}

impl Light {
    pub fn new(device: &wgpu::Device, layout:&wgpu::BindGroupLayout, ) -> Self {
        let padding = 0.0;
        let ambient = AmbientLight {
            color: [0.1, 0.1, 0.2, padding]
        };
        let point_lights = vec![PointLight::default(); 1000];
        let binding = create_lights_binding(device, layout, ambient, point_lights.as_slice());
        
        Light {
            ambient,
            point_lights: Vec::new(),
            binding,
        }
    }

    pub fn buffer_update(&mut self, gx: &WgpuGrapics) {
        gx.queue.write_buffer(
            &self.binding.buffers[0], 0, 
            bytemuck::cast_slice(&[self.ambient])
        );
        gx.queue.write_buffer(
            &self.binding.buffers[1], 0, 
            bytemuck::cast_slice(self.point_lights.as_slice())
        );
        gx.queue.write_buffer(
            &self.binding.buffers[2], 0, 
            bytemuck::cast_slice(&[self.point_lights.len() as u32])
        );
    }
}