use std::num::{NonZeroU8};
use image::{ImageBuffer, Rgba};
use wgpu::*;


pub struct WgpuTextureBinding {
    pub image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub size: Extent3d,
    pub texture: Texture,
    pub bindgroup: BindGroup,
    pub uv_scale: (f32, f32),
}

impl WgpuTextureBinding {

    pub fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        image_data: &[u8],
        uv_scale: (f32, f32),

    ) -> Self {

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let image = image::load_from_memory(image_data).unwrap();
        let image_buffer = image.to_rgba8();

        use image::GenericImageView;
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        //let uv_scale = [0.5; 2];

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[],
            }
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::default(),
                aspect: wgpu::TextureAspect::All,
            },
            &image_buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("texture_view"),
            format: Some(TextureFormat::Rgba8UnormSrgb),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None, //NonZeroU32::new(size.depth_or_array_layers),
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("texture_sampler"),
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: NonZeroU8::new(8),
            border_color: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
        });
        
        let bindgroup = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        
        Self {
            image_buffer,
            size,
            texture,
            bindgroup,
            uv_scale,
        }
    }
}

#[derive(Clone)]
pub enum TextureID {
    Index(usize),
    Null,
}

#[derive(Default)]
pub struct TexturePack {
    buffered_textures: Vec<WgpuTextureBinding>,
}

impl TexturePack {
    // pub fn new() -> Self {
    //     TexturePack { buffered_textures: Vec::new() }
    // }

    pub fn load(&mut self, device: &Device, queue: &Queue, image_data: &[u8], uv_scale: (f32, f32)) -> TextureID {
        self.buffered_textures.push(
            WgpuTextureBinding::new(device, queue, image_data, uv_scale)
        ); 
        TextureID::Index(self.buffered_textures.len() - 1)
    }

    pub fn get(&self, id: &TextureID) -> Option<&WgpuTextureBinding> {
        match id {
            TextureID::Index(idx) => Some(&self.buffered_textures[*idx]),
            TextureID::Null => None
        }
    }
}