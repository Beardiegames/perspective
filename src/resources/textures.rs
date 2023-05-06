use std::num::NonZeroU32;

use super::*;
use image::{ImageBuffer, Rgba};
use wgpu::*;


pub struct TexturePack {
    pub image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub size: Extent3d,
    pub texture: Texture,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
    pub render_pipeline_layout: PipelineLayout,
}

impl TexturePack {

    pub fn new(gx: &WgpuCore, bytes: &'static [u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        let image_buffer = image.to_rgba8();

        use image::GenericImageView;
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0 / 2,
            height: dimensions.1,
            depth_or_array_layers: 2,
        };

        let texture = gx.device.create_texture(
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

        gx.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image_buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0 / size.depth_or_array_layers),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("texture_view"),
            format: Some(TextureFormat::Rgba8UnormSrgb),
            dimension: Some(TextureViewDimension::D2Array),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: NonZeroU32::new(size.depth_or_array_layers),
        });

        let sampler = gx.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group_layout =
            gx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2Array,
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
        
            let bind_group = gx.device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout,
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
            
            let render_pipeline_layout = gx.device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout], // NEW!
                    push_constant_ranges: &[],
                }
            );

        Self {
            image_buffer,
            size,
            texture,
            bind_group_layout,
            bind_group,
            render_pipeline_layout,
        }
    }
}
