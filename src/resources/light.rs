use crate::{WgpuCore, WgpuDataBinding, create_effects_binding};


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    _padding: u32, // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here

    pub color: [f32; 3],
    _padding2: u32,

    pub ambient: [f32; 3],
    _padding3: u32,
}

pub struct Light {
    pub uniform: LightUniform,
    pub binding: WgpuDataBinding,
}

impl Light {
    pub fn new(device: &wgpu::Device, layout:&wgpu::BindGroupLayout, ) -> Self {
        let uniform = LightUniform {
            position: [2.0, 1.0, 2.0],
            _padding: 0,

            color: [1.0, 0.5, 0.25],
            _padding2: 0,

            ambient: [0.03, 0.05, 0.075],
            _padding3: 0,
        };
        let binding = create_effects_binding(device, layout, uniform);
        
        Light {
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