
//! Example:
//!``` rust
//! use perspective::*;
//!
//! fn main() -> anyhow::Result<()> {
//!     let window_size = PhysicalSize::new(1600, 1200);
//!     Perspective::new(window_size).run_winit::<RenderExample>()
//! }
//!
//! pub struct RenderExample {
//!     renderer: Renderer,
//! }
//!
//! impl PerspectiveHandler for RenderExample {
//!
//!     fn startup(gx: &mut WgpuCore) -> Self {
//!
//!         let mut textures = TexturePack::default();
//!
//!         let renderer = gx.setup_render_processor(
//!             &CameraSetup::default(),
//!             textures,
//!             &[]
//!         );
//!         RenderExample { renderer }
//!     }
//!
//!     fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { 
//!         false
//!     }
//!
//!     fn update(&mut self, _gx: &mut WgpuCore, px: &mut Perspective) {
//!         // application update code
//!     }
//! 
//! 
//!     fn resize(&mut self, width: u32, height: u32) {
//!         // handle window resize events
//!     }
//! 
//!     fn draw(&mut self, ctx: RenderContext) { 
//!         self.renderer.execute_render_pipeline(ctx);
//!     }
//! }
//!```

pub mod prelude;
mod shapes;
mod core;
mod renderer;
mod resources;
mod interface;
mod bindings;

use crate::core::*;
//use renderer::*;
//use resources::*;
use interface::*;
use wgpu::*;
use bindings::*;


pub struct Perspective {
    pub size: PhysicalSize<u32>,
    pub timer: RunTime,
    pub stop_running: bool,
}

pub use winit::{
    event::*,
	event_loop::{ControlFlow, EventLoop}, 
	window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};

impl Perspective {
    /// create a new Perspective instance
    /// @width & height: window size used for bulding window after run is called
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            timer: RunTime::new(),
            stop_running: false,
        }
    }

    /// return a tuple containing the (width, height) of the window
    pub fn window_size(&self) -> (u32, u32) { (self.size.width, self.size.height) }


    /// run the application in console only, don't draw a window
    pub fn run_cli<App>(mut self) -> anyhow::Result<()> 
        where App: 'static + PerspectiveHandler
    {
        let mut wgpu_core = core::WgpuCore::new::<Window>(None)?;
        let mut app = App::startup(&mut wgpu_core);

        while !self.stop_running {
            app.update(&mut wgpu_core, &mut self);

            let encoder = wgpu_core.device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            match self.render_pipe(&mut app, &mut wgpu_core, encoder) {
                Ok(_) => {},
               _ => eprintln!("Unexpected render error"),
            }
        }
        Ok(())
    }

    /// run the application by:
    /// setting up a new window, hooking up a gpu device 
    /// and starting a event based game loop
    pub fn run_winit<App>(mut self) -> anyhow::Result<()> 
        where App: 'static + PerspectiveHandler
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        
        let window_settings: WindowSettings<Window> = core::WindowSettings { 
            window: &window, 
            width: self.size.width, 
            height: self.size.height 
        };

        let mut wgpu_core = core::WgpuCore::new(Some(&window_settings))?;

        println!("-- perspective run:\n{:?}", wgpu_core.adapter.get_info());

        let mut app = App::startup(&mut wgpu_core);

        event_loop.run(move |event, _, control_flow| {
            self.event_handler(event, control_flow, &window, &mut wgpu_core, &mut app);
        });
    }

    /// is called by the eventloop everytime a new winit window event comes in
    fn event_handler<App>(
        &mut self, 
        event: Event<()>, 
        control_flow: &mut ControlFlow,
        window: &Window, 
        wgpu_core: &mut WgpuCore, 
        app: &mut App
    )
        where App: PerspectiveHandler
    {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } 
            if window_id == window.id() => self.window_event_handler(event, control_flow, app, wgpu_core),
    
            Event::RedrawRequested(window_id) if window_id == window.id() => self.update(control_flow, app, wgpu_core),
    
            Event::MainEventsCleared => self.redraw(window, ),
    
            _ => {}
        }
    }

    /// handles all window specific events:
    /// the input event for calling PerspectiveHandler intput
    /// basic application events for clean exit
    /// and window resize events
    fn window_event_handler<App>(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow, app: &mut App, wgpu_core: &mut WgpuCore)
        where App: PerspectiveHandler
    {
        if app.input(wgpu_core, event) { return; }

        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,

            WindowEvent::Resized(new_size) => self.resize(app, wgpu_core, new_size),

            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.resize(app, wgpu_core, new_inner_size),

            _ => {}
        };
    }

    /// tell all involved parties the window has been resized
    fn resize<App>(&mut self, app: &mut App, wgpu_core: &mut WgpuCore, new_size: &winit::dpi::PhysicalSize<u32>) 
        where App: PerspectiveHandler
    {
        self.size = *new_size;
        wgpu_core.resize(new_size.width, new_size.height);
        app.resize(new_size.width, new_size.height);
    }

    /// udpate calls PerspectiveHandler update methode
    /// after which we start the render pipeline
    fn update<App>(&mut self, control_flow: &mut ControlFlow, app: &mut App, wgpu_core: &mut WgpuCore)
        where App: PerspectiveHandler
    {
        app.update( wgpu_core, self);

        let encoder = wgpu_core.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        match self.render_pipe(app, wgpu_core, encoder) {
            Ok(_) => {},
            Err(PerspectiveError::SurfaceError(err)) => self.surface_error_handler(control_flow, app, wgpu_core, err),
            Err(PerspectiveError::NoCanvas) => eprintln!("No canvas to draw on. Should automatically be created at startup!"),
        }
    }

    /// handle surface errors
    /// Reconfigure the surface if lost by recalculating its size
    /// If the system is out of memory, we should probably quit
    /// All other errors (Outdated, Timeout) should be resolved by the next frame
    fn surface_error_handler<App>(
        &mut self, 
        control_flow: &mut ControlFlow, 
        app: &mut App, 
        wgpu_core: &mut WgpuCore, 
        err: SurfaceError
    )
        where App: PerspectiveHandler
    {
        match err {
            SurfaceError::Lost => self.resize(app, wgpu_core, &self.size.clone()),
            SurfaceError::OutOfMemory => *control_flow = ControlFlow::Exit, 
            _ => eprintln!("{:?}", err),
        }
    }

    /// render_pipe: prebuild surface texture and textureview before frame rendering
    fn render_pipe<App>(&mut self, app: &mut App, wgpu_core: &mut WgpuCore, encoder: CommandEncoder) -> Result<(), PerspectiveError>
        where App: PerspectiveHandler
    {

        return {
            app.draw(
                RenderContext{ 
                    px: self, 
                    gx: wgpu_core,
                    encoder, 
                    draw: wgpu_core.canvas.as_ref().map(|c| {
                        let output = c.surface
                            .get_current_texture()
                            .unwrap();

                        let view = output.texture.create_view(&TextureViewDescriptor::default());
                        let depth_map = &c.depth_map.view;
                        
                        DrawContext { view, depth_map, output }
                    })
                }
            );
            Ok(())
        };
    }


    /// redraw: tell winit to start drawing the next frame, and measure the frame duration of this one.
    /// RedrawRequested will only trigger once, unless we manually request it.
    fn redraw(&mut self, window: &Window) {
        window.request_redraw();
        self.timer.time_step();
    }
}
