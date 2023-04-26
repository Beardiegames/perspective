mod wgpu_core;

pub use wgpu_core::*;
pub use wgpu;

use winit::{
    event::*,
	event_loop::{ControlFlow, EventLoop}, 
	window::{WindowBuilder},
    dpi::PhysicalSize,
};


pub trait PerspectiveHandler {

    fn startup(gx: &mut WgpuCore) -> Self;

    #[allow(unused)]
    fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { false }

    #[allow(unused)]
    fn update(&mut self, gx: &mut WgpuCore) {}

    #[allow(unused)]
    fn resize(&mut self, width: u32, height: u32) {}

    fn render_pipeline(&mut self, gx: &WgpuCore, mut encoder: wgpu::CommandEncoder) -> Result<(), wgpu::SurfaceError> {
        if let Some(c) = &gx.canvas {

            let output = c.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            gx.quick_inject_render_passes(&view, &mut encoder); 
            gx.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }
        Ok(())
    }
}

pub struct Perspective {
    window_size: winit::dpi::PhysicalSize<u32>,
}

impl Perspective {

    pub fn new(width: u32, height: u32) -> Self {

        Perspective {
            window_size: PhysicalSize::new(width, height),
        }
    }

    pub fn run<App>(mut self) -> anyhow::Result<()> 
        where App: 'static + PerspectiveHandler
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        
        let window_settings = wgpu_core::WindowSettings { 
            window: &window, 
            width: self.window_size.width, 
            height: self.window_size.height 
        };
        let mut gx = wgpu_core::WgpuCore::new(Some(&window_settings))?;


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

                let encoder = gx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                match app.render_pipeline(&mut gx, encoder) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.resize(&mut app, &mut gx, &self.window_size.clone()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
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
}


