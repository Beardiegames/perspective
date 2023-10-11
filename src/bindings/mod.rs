mod buffer;
pub mod layout;

use wgpu::BindGroupLayout;
use wgpu::Device;
use crate::CameraUniform;
use crate::LightUniform;
use crate::SpriteAnimationData;
use crate::SpriteFrameElement;


pub struct WgpuBinding {
    pub buffers: Vec<wgpu::Buffer>,
    //pub layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
}

pub trait IntoBufferEntries {
    fn into_entries(&self) -> Vec<wgpu::BindGroupEntry>;
}

impl IntoBufferEntries for Vec<wgpu::Buffer> {
    fn into_entries(&self) -> Vec<wgpu::BindGroupEntry> {
        let mut entries = Vec::new();

        for i in 0..self.len() {
            entries.push(wgpu::BindGroupEntry {
                binding: i as u32,
                resource: self[i].as_entire_binding(),
            });
        }
        entries
    }
}


pub fn create_camera_binding(
    device: &Device,
    camera_layout: &BindGroupLayout,
    camera_uniform: CameraUniform
) -> WgpuBinding {

    let buffers = vec![buffer::create_camera_buffer(device, camera_uniform)];
    //let layout = layout::create_camera_layout(device);
    
    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: camera_layout,
        entries: buffers.into_entries().as_slice(),
        label: Some("Camera Bindgroup"),
    });

    WgpuBinding { buffers, bindgroup }
}

pub fn create_light_binding(
    device: &Device,
    light_layout: &BindGroupLayout,
    light_uniform: LightUniform
) -> WgpuBinding {

    let buffers = vec![buffer::create_light_buffer(device, light_uniform)];
    //let layout = layout::create_light_layout(device);

    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: light_layout,
        entries: buffers.into_entries().as_slice(),
        label: Some("Light Bindgroup"),
    });

    WgpuBinding { buffers, bindgroup }
}

pub fn create_sprite_animation_binding(
    device: &Device,
    sprite_layout: &BindGroupLayout,
    animations: &Vec::<SpriteAnimationData>,
    frames: &Vec::<SpriteFrameElement>,
    frames_passed: u32,
) -> WgpuBinding {

    let buffers = vec![
        buffer::create_sprite_frame_buffer(device, frames.clone()),
        buffer::create_sprite_timer_buffer(device, frames_passed),
        buffer::create_sprite_animation_buffer(device, animations.clone()),
    ];

    //let layout = layout::create_sprite_animation_layout(device);

    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Sprite Bindgroup"),
        layout: sprite_layout,
        entries: buffers.into_entries().as_slice()
    });

    WgpuBinding { buffers, bindgroup }
}



