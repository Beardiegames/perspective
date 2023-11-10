use super::*;

pub enum PerspectiveError {
    SurfaceError(wgpu::SurfaceError),
    NoCanvas,
}

pub trait PerspectiveHandler {

    #[allow(unused)]
    fn setup(sys: PerspectiveSystem) -> Self;

    #[allow(unused)]
    fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { false }

    #[allow(unused)]
    fn update(&mut self, sys: PerspectiveSystem) {} //gx: &mut WgpuCore, px: &mut Perspective) {}

    #[allow(unused)]
    fn resize(&mut self, width: u32, height: u32) {}

    // #[allow(unused)]
    // fn draw(&mut self, mut ctx: RenderContext) {}  
}

pub struct PerspectiveSystem<'a> {
    pub gx: &'a mut WgpuCore,
    pub rnd: &'a mut Renderer,
}

impl<'a> PerspectiveSystem<'a> {

    pub fn timer(&mut self) -> &mut RunTime {
        &mut self.gx.timer
    }

    pub fn camera(&mut self) -> &mut Camera {
        &mut self.rnd.camera
    }

    pub fn set_camera_position(&mut self, x:f32, y:f32, z:f32) {
        self.rnd.camera.eye = cgmath::Point3::new(x, y, z);
    }

    pub fn set_camera_target(&mut self,  x:f32, y:f32, z:f32) {
        self.rnd.camera.target = cgmath::Point3::new(x, y, z);
    }

    pub fn load_texture(
        &mut self, 
        image_data: &[u8], 
        uv_scale: (f32, f32), 
        pool_settings: Option<SpritePoolSettings>
        ) -> SpritePoolID 
    {
        let texture_id = self.rnd.assets.load_texture(
            &self.gx.device, 
            &self.gx.queue, 
            image_data, 
            uv_scale
        );

        self.rnd.create_sprite_pool(
            self.gx,
            &texture_id,
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
        rotation: cgmath::Quaternion<f32>
        ) -> SpriteInstanceID 
    {
        self.rnd.sprites.spawn_sprite(sprite_pool_id, position, rotation)
    }

    pub fn get_instance(
        &mut self, 
        instance_id: &SpriteInstanceID
        ) -> &mut ObjectInstance 
    {
        self.rnd.sprites.get_instance(instance_id)
    }
}


pub struct RenderContext<'a> {
    //pub px: &'a Perspective,
    //pub gx: &'a WgpuCore,
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

