
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
use interface::*;
use wgpu::*;
use bindings::*;

pub use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop}, 
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};

#[derive(Default)]
pub struct PerspectiveBuilder {
    pub size: PhysicalSize<u32>,
    pub camera_setup: CameraSetup,
}

impl PerspectiveBuilder {
    pub fn new() -> Self {
        PerspectiveBuilder::default()
    }

    pub fn set_window_size(mut self, size: PhysicalSize<u32>) -> Self {
        self.size = size;
        self
    }

    pub fn set_camera(mut self, camera_setup: CameraSetup) -> Self {
        self.camera_setup = camera_setup;
        self
    }

    pub fn run<App>(self) -> anyhow::Result<()> 
        where App: 'static + PerspectiveHandler 
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let wgpu_core = core::WgpuCore::new(&window, self.size)?;
        let renderer = Renderer::new(&wgpu_core.device, &self.camera_setup, AssetPack::default()); 

        let perspective = Perspective {
            size: self.size,
            camera_setup: self.camera_setup,
            wgpu_core,
            window,
            stop_running: false,
            renderer,
        };

        perspective.run::<App>(event_loop)
    }
}

pub struct Perspective {
    pub size: PhysicalSize<u32>,
    pub camera_setup: CameraSetup,
    pub wgpu_core: WgpuCore,
    pub window: Window,
    pub stop_running: bool,
    pub renderer: Renderer,
}

impl Perspective {

    /// return a tuple containing the (width, height) of the window
    pub fn window_size(&self) -> (u32, u32) { (self.size.width, self.size.height) }

    /// run the application by:
    /// setting up a new window, hooking up a gpu device 
    /// and starting a event based game loop
    fn run<App>(mut self, event_loop: EventLoop<()>) -> anyhow::Result<()> 
        where App: 'static + PerspectiveHandler
    {
        println!("-- perspective run:\n{:?}", self.wgpu_core.adapter.get_info());

        let mut app = App::setup(PerspectiveSystem { gx: &mut self.wgpu_core, rnd: &mut self.renderer});
        event_loop.run(move |event, _, control_flow| self.event_handler(event, control_flow, &mut app));
    }

    // -- winit draw system --

    /// is called by the eventloop everytime a new winit window event comes in
    fn event_handler<App>(
        &mut self, 
        event: Event<()>, 
        control_flow: &mut ControlFlow,
        //window: &Window, 
        //wgpu_core: &mut WgpuCore, 
        app: &mut App
    )
        where App: PerspectiveHandler
    {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } 
            if window_id == self.window.id() => self.window_event_handler(event, control_flow, app),
    
            Event::RedrawRequested(window_id) if window_id == self.window.id() => self.update(control_flow, app),
            Event::MainEventsCleared => self.redraw(),
            _ => {}
        }
    }

    /// handles all window specific events:
    /// the input event for calling PerspectiveHandler intput
    /// basic application events for clean exit
    /// and window resize events
    fn window_event_handler<App>(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow, app: &mut App)
        where App: PerspectiveHandler
    {
        if app.input(&mut self.wgpu_core, event) { return; }

        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,

            WindowEvent::Resized(new_size) => self.resize(app, new_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self.resize(app, new_inner_size),
            _ => {}
        };
    }

    /// tell all involved parties the window has been resized
    fn resize<App>(&mut self, app: &mut App, new_size: &winit::dpi::PhysicalSize<u32>) 
        where App: PerspectiveHandler
    {
        self.size = *new_size;
        self.wgpu_core.resize(new_size.width, new_size.height);
        app.resize(new_size.width, new_size.height);
    }

    /// udpate calls PerspectiveHandler update methode
    /// after which we start the render pipeline
    fn update<App>(&mut self, control_flow: &mut ControlFlow, app: &mut App)
        where App: PerspectiveHandler
    {
        app.update(PerspectiveSystem { gx: &mut self.wgpu_core, rnd: &mut self.renderer });// wgpu_core, self);

        let encoder = self.wgpu_core.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        match self.render_pipe(app, encoder) {
            Ok(_) => {},
            Err(PerspectiveError::SurfaceError(err)) => self.surface_error_handler(control_flow, app, err),
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
        err: SurfaceError
    )
        where App: PerspectiveHandler
    {
        match err {
            SurfaceError::Lost => self.resize(app, &self.size.clone()),
            SurfaceError::OutOfMemory => *control_flow = ControlFlow::Exit, 
            _ => eprintln!("{:?}", err),
        }
    }

    /// render_pipe: prebuild surface texture and textureview before frame rendering
    fn render_pipe<App>(&mut self, _app: &mut App, encoder: CommandEncoder) -> Result<(), PerspectiveError>
        where App: PerspectiveHandler
    {
        return {
            self.renderer.execute_render_pipeline(
                &self.wgpu_core,
                RenderContext{ 
                    encoder, 
                    draw: self.wgpu_core.canvas.as_ref().map(|c| {
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
    fn redraw(&mut self) {
        self.window.request_redraw();
        self.wgpu_core.timer.time_step();
    }
}
