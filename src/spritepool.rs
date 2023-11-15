use crate::{layout::PerspectiveShaderLayout};
use super::*;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;

pub struct SpriteRenderObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub texture_id: AssetID,
    pub animation: SpriteGpuHandle,
}

impl SpriteRenderObject {
    pub fn new(
        device: &Device, 
        bind_group_layouts: &PerspectiveShaderLayout,
        texture_id: AssetID,
        settings: &SpritePoolSettings,
        ) -> Self 
    {
        // Setup vertex bindings
        let shape = crate::shapes::create_square([settings.tile_size.0, settings.tile_size.1], settings.image_aspect);
        //let shape = crate::shapes::create_hexagon([settings.tile_size.0, settings.tile_size.1], settings.image_aspect);
        let (vertex_buffer, index_buffer) = shape.setup_wgpu_buffers(device);

        let num_vertices = shape.vertices.len() as u32;
        let num_indices = shape.indices.len() as u32;

        // Setup sprite animation bindings
        let animation = SpriteGpuHandle::new(
            device, 
            bind_group_layouts.sprite_layout(),
            settings.animation_frames.clone(),
            settings.max_pool_size //instances.len()
        );

        SpriteRenderObject {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            texture_id,
            animation,
        }
    }
}

#[derive(Clone)]
pub struct SpritePoolSettings {
    pub image_aspect: f32,
    pub tile_size: (f32, f32),
    pub animation_frames: Vec<[f32; 2]>,
    pub max_pool_size: usize,
}

impl Default for SpritePoolSettings {
    fn default() -> Self {
        SpritePoolSettings {
            image_aspect: 1.0,
            tile_size: (1.0, 1.0),
            animation_frames: vec![[0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]],
            max_pool_size: 100_000,
        }
    }
}

pub struct SpritePoolID {
    index: usize,
}

pub struct SpriteInstanceID {
    pool_idx: usize,
    instance_idx: usize,
}

pub struct SpritePool {
    pub sprite_obj: SpriteRenderObject,
    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,

    pub num_spawns: usize,
}

impl SpritePool {

    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &PerspectiveShaderLayout,
        texture_id: AssetID,
        settings: &SpritePoolSettings,
        ) -> Self 
    {
        let sprite_obj = SpriteRenderObject::new(
            device, //queue, 
            bind_group_layouts,
            texture_id,
            settings,
        );

        let instances = (0..settings.max_pool_size as u32)
            .map(|instance_idx| {
                ObjectInstance { 
                    instance_idx, 
                    position: cgmath::Vector3::zero(), 
                    rotation: cgmath::Quaternion::zero(), 
                    scale: 1., 
                }
            })
            .collect::<Vec<_>>();                  

        let instance_data = instances.iter().map(ObjectInstance::to_raw).collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        SpritePool {
            sprite_obj,
            instances,
            instance_buffer,

            num_spawns: 0
        }
    }

    pub fn update_instance_buffer(&mut self, gx: &WgpuGrapics) {
        let instance_data = self.instances.iter().map(ObjectInstance::to_raw).collect::<Vec<_>>();

        gx.queue.write_buffer(
            &self.instance_buffer, 
            0, 
            bytemuck::cast_slice(&instance_data),
        );
    }
}

#[derive(Default)]
pub struct Sprites {
    sprite_pools: Vec<SpritePool>,
}

impl Sprites {
    pub fn new() -> Self {
        Sprites::default()
    }

    pub fn mut_pool_list(&mut self) -> &mut Vec<SpritePool> {
        &mut self.sprite_pools
    }

    pub fn add_sprite_pool(&mut self, sprite_pool: SpritePool) -> SpritePoolID {
        self.sprite_pools.push(sprite_pool);
        SpritePoolID { index: self.sprite_pools.len() - 1 }
    }

    pub fn spawn_sprite(
        &mut self,
        sprite_pool_id: &SpritePoolID,
        position: cgmath::Vector3<f32>,
        rotation: cgmath::Quaternion<f32>,
        scale: f32,
        ) -> SpriteInstanceID 
    {
        let instance_idx = self.sprite_pools[sprite_pool_id.index].num_spawns;
        let sprite = &mut self.sprite_pools[sprite_pool_id.index].instances[instance_idx];
            sprite.position = position;
            sprite.rotation = rotation;
            sprite.scale = scale;

        self.sprite_pools[sprite_pool_id.index].num_spawns += 1;

        SpriteInstanceID { 
            pool_idx: sprite_pool_id.index, 
            instance_idx: self.sprite_pools[sprite_pool_id.index].num_spawns - 1
        }
    }

    pub fn get_instance(&mut self, sprite_instance_id: &SpriteInstanceID) -> &mut ObjectInstance {
        &mut self.sprite_pools[sprite_instance_id.pool_idx].instances[sprite_instance_id.instance_idx]
    }
}