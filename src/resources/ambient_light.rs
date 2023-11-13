use crate::{WgpuGrapics, WgpuDataBinding, create_lights_binding};


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AmbientLightUniform {
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLightData {
    pub position: [f32; 4],
    pub color: [f32; 4],
    //_padding: u32, // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    //pub range: f32,
}

pub struct Light {
    pub ambient: AmbientLightUniform,
    pub point_lights: Vec<PointLightData>,
    pub binding: WgpuDataBinding,
}

impl Light {
    pub fn new(device: &wgpu::Device, layout:&wgpu::BindGroupLayout, ) -> Self {
        let padding = 0.0;
        let ambient = AmbientLightUniform {
            color: [0.1, 0.1, 0.2, padding]
        };
        let point_lights = vec![
            PointLightData {
                position: [0.0, 0.0, 45.0, 1.0],
                color: [0.95, 0.2, 0.0, padding],
                //range: 10.0,
            }
        ];
        let binding = create_lights_binding(device, layout, ambient, point_lights.as_slice());
        
        Light {
            ambient,
            point_lights,
            binding,
        }
    }

    pub fn buffer_update(&mut self, gx: &WgpuGrapics) {
        gx.queue.write_buffer(
            &self.binding.buffers[0], 
            0, 
            bytemuck::cast_slice(&[self.ambient])
        );

        gx.queue.write_buffer(
            &self.binding.buffers[1], 
            0, 
            bytemuck::cast_slice(self.point_lights.as_slice())
        );
    }

}