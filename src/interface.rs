use super::*;


pub enum PerspectiveError {
    SurfaceError(wgpu::SurfaceError),
    NoCanvas,
}


pub trait PerspectiveHandler {

    fn startup(gx: &mut WgpuCore) -> Self;

    #[allow(unused)]
    fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { false }

    #[allow(unused)]
    fn update(&mut self, gx: &mut WgpuCore, px: &mut Perspective) {}

    #[allow(unused)]
    fn resize(&mut self, width: u32, height: u32) {}

    #[allow(unused)]
    fn draw(&mut self, mut ctx: RenderContext) {}
    
}


pub struct RenderContext<'a> {
    pub px: &'a Perspective,
    pub gx: &'a WgpuCore,
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
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &drw.depth_map,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                })
            ),
            None => None,
        }
    }
}

