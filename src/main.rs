mod gfx;

use winit::{
    event::*,
	event_loop::{ControlFlow, EventLoop}, 
	window::{WindowBuilder, Window}
};

impl gfx::ComputeData for u32 {
	fn from_bytes(b: &[u8]) -> u32 {
		let input = &mut b.clone();
		let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
	    *input = rest;
	    u32::from_ne_bytes(int_bytes.try_into().unwrap())
	}
}


fn main() -> anyhow::Result<()> {

    // create winit application window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    //let this_window_id = window.id();
    let window_settings = gfx::WindowSettings { window: &window, width: 800, height: 600 };
    let wgpu_core = gfx::WgpuCore::new(Some(&window_settings))?;
    //drop(window_settings);

    let mut game = Game::new(wgpu_core);

    // run
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } 
        if window_id == window.id() => if !game.input(event) {
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

                WindowEvent::Resized(physical_size) => game.resize(physical_size),

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => game.resize(new_inner_size),

                _ => {}
            }
        },

        Event::RedrawRequested(window_id) if window_id == window.id() => {
            game.update();
            match game.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                //Err(wgpu::SurfaceError::Lost) => game.resize(game.size),
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

pub struct Game {
    wgpu_core: gfx::WgpuCore,
}

impl Game {
    pub fn new(wgpu_core: gfx::WgpuCore) -> Self {
            // create wgpu handle
        

        // log to check if everything is going well
        println!("{:?}", wgpu_core.adapter.get_info());

        // // setup compute data
        // let data = vec![1, 2, 3, 4];
        
        // // setup compute shader handles
        // let compute_processor = gfx::ComputeProcessor::new(
        //     &mut wgpu_core, 
        //     include_str!("shaders/compute_shader.wgsl"), 
        //     data
        // );

        // // build compute shader
        // let cbuff = compute_processor.execute(&wgpu_core);

        // // Poll the device in a blocking manner so that our future resolves.
        // wgpu_core.device.poll(wgpu::Maintain::Wait); 

        // let udat = compute_processor.post_render(cbuff);

        // for val in &udat {
        //     println!("val: {}", val);
        // }

        Game {
            wgpu_core,
        }
    }

    fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        self.wgpu_core.resize(new_size.width, new_size.height);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        //todo!()
        false
    }

    fn update(&mut self) {
        //todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let Some(c) = &self.wgpu_core.canvas {

            let output = c.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = self.wgpu_core.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
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
                });
            }

            // submit will accept anything that implements IntoIter
            self.wgpu_core.queue.submit(std::iter::once(encoder.finish()));
            output.present(); 
        }

        Ok(())
    }
}


