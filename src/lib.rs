mod core;
mod builder;
mod processors;
mod resources;
pub mod shapes;

pub use builder::*;
pub use crate::core::*;
pub use processors::*;
pub use resources::*;
pub use wgpu::*;

use winit::{
    event::*,
	event_loop::{ControlFlow, EventLoop}, 
	window::{WindowBuilder},
    dpi::PhysicalSize,
};

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

pub enum PerspectiveError {
    SurfaceError(wgpu::SurfaceError),
    NoCanvas,
}

pub trait PerspectiveHandler {

    fn startup(gx: &mut WgpuCore) -> Self;

    #[allow(unused)]
    fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { false }

    #[allow(unused)]
    fn update(&mut self, gx: &mut WgpuCore) {}

    #[allow(unused)]
    fn resize(&mut self, width: u32, height: u32) {}

    #[allow(unused)]
    fn render_pipeline(&mut self, gx: &WgpuCore, mut render: RenderContext) {
        
        render.begin_render_pass();

        gx.queue.submit(Some(render.encoder.finish()));
        render.output.present();
    }
}

pub struct Perspective {
    window_size: winit::dpi::PhysicalSize<u32>,
}

impl Perspective {

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            window_size: PhysicalSize::new(width, height),
        }
    }

    pub fn run<App>(mut self) -> anyhow::Result<()> 
        where App: 'static + PerspectiveHandler
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        
        let window_settings: WindowSettings<winit::window::Window> = core::WindowSettings { 
            window: &window, 
            width: self.window_size.width, 
            height: self.window_size.height 
        };
        let mut gx = core::WgpuCore::new(Some(&window_settings))?;


        println!("-- perspective run:\n{:?}", gx.adapter.get_info());
        

        let mut app = App::startup(&mut gx);


        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } 
            if window_id == window.id() => if !app.input(&mut gx, event) {
                match event {
    
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
    
                    WindowEvent::Resized(physical_size) => self.resize(&mut app, &mut gx, physical_size),
    
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.resize(&mut app, &mut gx, new_inner_size),
    
                    _ => {}
                }
            },
    
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                app.update(&mut gx);

                let encoder = gx.device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                match self.render_pipe(&mut app, &mut gx, encoder) {
                    Ok(_) => {},
                    Err(PerspectiveError::SurfaceError(e)) => {
                        match e {
                            // Reconfigure the surface if lost
                            SurfaceError::Lost => self.resize(&mut app, &mut gx, &self.window_size.clone()),
                            // The system is out of memory, we should probably quit
                            SurfaceError::OutOfMemory => *control_flow = ControlFlow::Exit,
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            _ => eprintln!("{:?}", e),
                        }
                    },
                    Err(PerspectiveError::NoCanvas) => eprintln!("No canvas to draw on. Should automatically be created at startup!"),
                }
            },
    
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it.
                window.request_redraw();
            },
    
            _ => {}
        });

        //Ok(())
    }

    fn resize<App>(&mut self, app: &mut App, gx: &mut WgpuCore, new_size: &winit::dpi::PhysicalSize<u32>) 
    where App: PerspectiveHandler
    {
        self.window_size = *new_size;
        gx.resize(new_size.width, new_size.height);
        app.resize(new_size.width, new_size.height);
    }

    fn render_pipe<App>(&mut self, app: &mut App, gx: &mut WgpuCore, encoder: CommandEncoder) -> Result<(), PerspectiveError>
    where App: PerspectiveHandler
    {
        if let Some(c) = &gx.canvas {

            let output = c.surface
                .get_current_texture()
                .map_err(|e| PerspectiveError::SurfaceError(e))?;

            let view = output.texture.create_view(&TextureViewDescriptor::default());
            
            return Ok(app.render_pipeline(gx, RenderContext{ encoder, view, output })); 
        }
        Err(PerspectiveError::NoCanvas)
    }
}


