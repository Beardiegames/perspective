// use crate::{WgpuGrapics, WgpuDataBinding, create_pointlight_binding};


// #[repr(C)]
// #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct PointLightData {
//     pub position: [f32; 4],
//     pub color: [f32; 4],
//     //_padding: u32, // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
//     pub range: f32,
// }

// pub struct PointLight {
//     pub uniform: PointLightData,
//     pub binding: WgpuDataBinding,
// }

// impl PointLight {
//     pub fn new(device: &wgpu::Device, layout:&wgpu::BindGroupLayout, ) -> Self {
//         let uniform = PointLightUniform {
//             position: [0.0, 0.0, 0.0, 0.0],
//             color: [0.03, 0.05, 0.075, 0.0],
//             range: 10.0,
//         };
//         let binding = create_pointlight_binding(device, layout, uniform);
        
//         PointLight {
//             uniform,
//             binding,
//         }
//     }

//     pub fn buffer_update(&mut self, gx: &WgpuGrapics) {
//         // let old_position: cgmath::Vector3<_> = self.uniform.position.into();

//         // let new_position = cgmath::Quaternion::from_axis_angle(
//         //         (0.0, 1.0, 0.0).into(), 
//         //         cgmath::Deg(1.0)
//         //     ) * old_position;

//         // self.uniform.position = new_position.into();

//         gx.queue.write_buffer(
//             &self.binding.buffers[0], 
//             0, 
//             bytemuck::cast_slice(&[self.uniform])
//         );
//     }

// }