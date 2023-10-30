use crate::{WgpuCore, WgpuDataBinding, create_lights_binding};


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AmbientLightUniform {
    pub direction: [f32; 3],
    _padding: u32, // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here

    pub light_color: [f32; 3],
    _padding2: u32,

    pub shadow_color: [f32; 3],
    _padding3: u32,
}

pub struct AmbientLight {
    pub uniform: AmbientLightUniform,
    pub binding: WgpuDataBinding,
}

impl AmbientLight {
    pub fn new(device: &wgpu::Device, layout:&wgpu::BindGroupLayout, ) -> Self {
        let uniform = AmbientLightUniform {
            direction: [0.0, 1.0, 0.0],
            _padding: 0,

            light_color: [1.0, 0.5, 0.25],
            _padding2: 0,

            shadow_color: [0.03, 0.05, 0.075],
            _padding3: 0,
        };
        let binding = create_lights_binding(device, layout, uniform);
        
        AmbientLight {
            uniform,
            binding,
        }
    }

    pub fn buffer_update(&mut self, gx: &WgpuCore) {
        // let old_position: cgmath::Vector3<_> = self.uniform.position.into();

        // let new_position = cgmath::Quaternion::from_axis_angle(
        //         (0.0, 1.0, 0.0).into(), 
        //         cgmath::Deg(1.0)
        //     ) * old_position;

        // self.uniform.position = new_position.into();

        gx.queue.write_buffer(
            &self.binding.buffers[0], 
            0, 
            bytemuck::cast_slice(&[self.uniform])
        );
    }

}