use wgpu::{Device, Buffer, util::{DeviceExt, BufferInitDescriptor}};

use crate::{
    CameraUniform, 
    AmbientLightUniform, 
    SpriteAnimationData, 
    SpriteFrameElement
};


pub fn create_camera_buffer(device: &Device, camera_uniform: CameraUniform) -> Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    )
}


pub fn create_lights_buffer(device: &Device, ambient_light_uniform: AmbientLightUniform) -> Buffer {
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&[ambient_light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    )
}

pub fn create_sprite_animation_buffer(device: &Device, animation_instances: Vec::<SpriteAnimationData>) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some(&format!("Sprite Animation Buffer")),
        contents: bytemuck::cast_slice(animation_instances.as_slice()),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
    })
}

pub fn create_sprite_frame_buffer(device: &Device, spriteframes: Vec<SpriteFrameElement>) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some(&format!("Sprite Frames Buffer")),
        contents: bytemuck::cast_slice(spriteframes.as_slice()),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
    })
}

pub fn create_sprite_timer_buffer(device: &Device, frames_passed: u32) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some(&format!("Sprite Timer Buffer")),
        contents: bytemuck::cast_slice(&[frames_passed]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
    })
}



