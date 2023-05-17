use super::*;
use std::time::Instant;


pub enum PerspectiveError {
    SurfaceError(wgpu::SurfaceError),
    NoCanvas,
}


pub trait PerspectiveHandler {

    fn startup(gx: &mut WgpuCore) -> Self;

    #[allow(unused)]
    fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { false }

    #[allow(unused)]
    fn update(&mut self, gx: &mut WgpuCore, px: &Perspective) {}

    #[allow(unused)]
    fn resize(&mut self, width: u32, height: u32) {}

    #[allow(unused)]
    fn render_pipeline(&mut self, gx: &WgpuCore, mut render: RenderContext) {
        
        render.begin_render_pass();

        gx.queue.submit(Some(render.encoder.finish()));
        render.output.present();
    }
}


pub struct RunTime {
    instant: Instant,
    previous: u128,
    elapsed: u128,
    frame_delta: f64,
}

impl RunTime {
    pub fn new() -> Self {
        RunTime {
            instant: Instant::now(),
            previous: 0,
            elapsed: 0,
            frame_delta: 0.0,
        }
    }

    pub fn time_step(&mut self) {
        self.previous = self.elapsed;
        self.elapsed = self.instant.elapsed().as_micros();
        self.frame_delta = (self.elapsed - self.previous) as f64 / 1_000_000.0;
    }

    pub fn elapsed(&self) -> u128 { self.elapsed }

    pub fn frame_delta(&self) -> f64 { self.frame_delta }
}


pub struct RenderContext {
    pub encoder: CommandEncoder, 
    pub view: TextureView, 
    pub output: SurfaceTexture,
}

impl RenderContext {
    pub fn begin_render_pass(&mut self) -> RenderPass {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
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
            depth_stencil_attachment: None,
        })
    }
}


