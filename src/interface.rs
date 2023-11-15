use super::*;
use crate::spritepool::*;
use cgmath::*;

pub enum PerspectiveError {
    SurfaceError(wgpu::SurfaceError),
    NoCanvas,
}

pub trait Perspective {

    #[allow(unused)]
    fn setup(ctl: ControlPanel) -> Self;

    #[allow(unused)]
    fn input(&mut self, ctl: ControlPanel, event: &WindowEvent) { }

    #[allow(unused)]
    fn update(&mut self, ctl: ControlPanel) {} //gx: &mut WgpuGrapics, px: &mut Perspective) {}

    #[allow(unused)]
    fn resize(&mut self, width: u32, height: u32) {}
}

pub struct ControlPanel<'a> {
    pub gfx: &'a mut WgpuGrapics,
    pub draw: &'a mut Renderer,
}

impl<'a> ControlPanel<'a> {

    pub fn timer(&self) -> &RunTime { &self.gfx.timer }

    pub fn camera(&mut self) -> &mut Camera { &mut self.draw.camera }

    pub fn light(&mut self) -> &mut Light { &mut self.draw.light }

    // pub fn set_camera_position(&mut self, x:f32, y:f32, z:f32) {
    //     self.draw.camera.eye = cgmath::Point3::new(x, y, z);
    // }

    // pub fn set_camera_target(&mut self,  x:f32, y:f32, z:f32) {
    //     self.draw.camera.target = cgmath::Point3::new(x, y, z);
    // }

    pub fn create_sprite_pool(
        &mut self, 
        image_data: &[u8],
        pool_settings: Option<SpritePoolSettings>
        ) -> SpritePoolID 
    {
        let tex_id = self.draw.assets.load_texture(
            &self.gfx.device, 
            &self.gfx.queue, 
            image_data, 
            match &pool_settings { 
                Some(s) => s.tile_size,
                None => (1.0, 1.0),
            }
        );
        self.draw.create_sprite_pool(
            self.gfx,
            &tex_id,
            &match pool_settings {
                Some(s) => s,
                None => SpritePoolSettings::default(),
            }
        )
    }

    pub fn spawn_sprite(
        &mut self, 
        sprite_pool_id: &SpritePoolID,
        position: cgmath::Vector3<f32>,
        ) -> SpriteInstanceID 
    {
        self.draw.sprites.spawn_sprite(
            sprite_pool_id, 
            position,
            Quaternion::from_axis_angle(Vector3::unit_z(), Deg(0.0)), 
            1.
        )
    }

    pub fn spawn_sprite_transform(
        &mut self, 
        sprite_pool_id: &SpritePoolID,
        position: cgmath::Vector3<f32>,
        rotation: cgmath::Quaternion<f32>,
        scale: f32,
        ) -> SpriteInstanceID 
    {
        self.draw.sprites.spawn_sprite(sprite_pool_id, position, rotation, scale)
    }

    pub fn get_instance(
        &mut self, 
        instance_id: &SpriteInstanceID
        ) -> &mut ObjectInstance 
    {
        self.draw.sprites.get_instance(instance_id)
    }
}

pub struct RenderContext<'a> {
    pub encoder: CommandEncoder,
    pub draw: Option<DrawContext<'a>>,
}

pub struct DrawContext<'a> {
    pub view: TextureView, 
    pub depth_map: &'a TextureView, 
    pub output: SurfaceTexture,
}

impl<'a> RenderContext<'a> {
    pub fn begin_render_pass(&mut self) -> Option<RenderPass> {

        match &self.draw {
            Some(drw) => Some(
                self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &drw.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: drw.depth_map,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    occlusion_query_set: None, // FIX: might be an issue
                    timestamp_writes: None, // FIX: might be an issue
                }
            )),
            None => None,
        }
    }
}

