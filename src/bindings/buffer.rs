use wgpu::{Device, Buffer, util::{DeviceExt, BufferInitDescriptor}};

use crate::{
    CameraUniform, 
    AmbientLightUniform, 
    PointLightData, 
    SpriteAnimationData,
    SpriteFrameElement
};


pub fn create_camera_buffer(device: &Device, camera_uniform: CameraUniform) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn create_ambientlight_buffer(device: &Device, ambient_uniform: AmbientLightUniform) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Ambient Light Buffer"),
        contents: bytemuck::cast_slice(&[ambient_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn create_pointlight_buffer(device: &Device, pointlights: &[PointLightData]) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Point Light Buffer"),
        contents: bytemuck::cast_slice(pointlights),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn create_sprite_animation_buffer(device: &Device, animation_instances: &[SpriteAnimationData]) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Sprite Animation Buffer"),
        contents: bytemuck::cast_slice(animation_instances),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
    })
}

pub fn create_sprite_frame_buffer(device: &Device, spriteframes: &[SpriteFrameElement]) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Sprite Frames Buffer"),
        contents: bytemuck::cast_slice(spriteframes),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, 
    })
}

pub fn create_sprite_timer_buffer(device: &Device, frames_passed: u32) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Sprite Timer Buffer"),
        contents: bytemuck::cast_slice(&[frames_passed]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
    })
}



