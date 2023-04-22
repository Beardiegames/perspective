mod wgpu_core;

pub use wgpu_core::*;
pub use wgpu;

use winit::{
    event::*,
	event_loop::{ControlFlow, EventLoop}, 
	window::{WindowBuilder, Window},
    dpi::PhysicalSize,
};


pub trait PerspectiveHandler  
//where <Self as PerspectiveHandler>::ComputeType: ComputeData 
{
    //type ComputeType;

    fn startup(gx: &mut WgpuCore) -> Self;

    fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { false }

    fn update(&mut self, gx: &mut WgpuCore) {}

    fn resize(&mut self, width: u32, height: u32) {}

    // fn compute_processor(&mut self) -> Option<&ComputeProcessor<Self::ComputeType>> { None }

    // fn processors_updated(&mut self, result_data: Vec<Self::ComputeType>) {}

    fn render_pipeline(&mut self, gx: &WgpuCore, mut encoder: wgpu::CommandEncoder) -> Result<(), wgpu::SurfaceError> {
        if let Some(c) = &gx.canvas {

            let output = c.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            gx.inject_basic_render_passes(&view, &mut encoder); 
            gx.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }
        Ok(())
    }

    // fn execute_processors<F>(&mut self, gfx: &WgpuCore, on_post_render: F) 
    // where F: FnOnce(&mut Self) -> () 
    // { }

    // fn post_render_update<'a>(&mut self, compute_buffer: BufferSlice) {}
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

                match self.render(&mut app, &gx) {
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
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            },
    
            _ => {}
        });

        Ok(())
    }

    fn resize<App>(&mut self, app: &mut App, gx: &mut WgpuCore, new_size: &winit::dpi::PhysicalSize<u32>) 
    where App: PerspectiveHandler
    {
        gx.resize(new_size.width, new_size.height);
        self.window_size = *new_size;
        app.resize(new_size.width, new_size.height);
    }

    fn render<App>(&mut self, app: &mut App, gx: &WgpuCore) -> Result<(), wgpu::SurfaceError> 
    where App: 'static + PerspectiveHandler
    {
    	let mut encoder = gx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        	label: Some("Render Encoder"),
        });

        app.render_pipeline(gx, encoder);
                
        // if let Some(c) = &gx.canvas {

        //     let output = c.surface.get_current_texture()?;
        //     let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        //     {
        //         let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //             label: Some("Render Pass"),
        //             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        //                 view: &view,
        //                 resolve_target: None,
        //                 ops: wgpu::Operations {
        //                     load: wgpu::LoadOp::Clear(wgpu::Color {
        //                         r: 0.1,
        //                         g: 0.2,
        //                         b: 0.3,
        //                         a: 1.0,
        //                     }),
        //                     store: true,
        //                 },
        //             })],
        //             depth_stencil_attachment: None,
        //         });
        //     } 

        //     match app.compute_processor() {
        //         Some(p) => {
        //             p.inject_passes(&mut encoder);

        //             // submit will accept anything that implements IntoIter
        //             gx.queue.submit(std::iter::once(encoder.finish()));
                    
        //             let buffer = p.slice_staging_buffer();

        //             gx.device.poll(wgpu::Maintain::Wait); 
                    
        //             let udat = p.post_render(buffer);
        //             app.processors_updated(udat);
        //         },
        //         None => {
        //             gx.queue.submit(std::iter::once(encoder.finish()));
        //         }
        //     };
        //     output.present();
        // }
        // else {
        //     if let Some(p) = app.compute_processor() {
        //         let buffer = p.execute(&gx);

        //         gx.device.poll(wgpu::Maintain::Wait); 

        //         let udat = p.post_render(buffer);
        //         app.processors_updated(udat);
        //     }
        // }
        
        // app.processors_updated(p_buffer);

        Ok(())
    }
}


