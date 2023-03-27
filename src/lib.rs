mod canvas;
pub use canvas::*;

mod shaders;
pub use shaders::*;

mod render_pipeline;
pub use render_pipeline::*;

mod state;
pub use state::*;

mod vertices;
pub use vertices::*;

mod texture;
pub use texture::*;

mod camera;
pub use camera::*;

mod instances;
pub use instances::*;

// mod model;
// pub use model::*;

// mod resources;
// pub use resources::*;

pub use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder, WindowId},
};
use pollster::block_on;
pub use state::Renderer;


pub trait GfxHandler {
    fn on_input(&mut self, event: &WindowEvent) -> bool;
    
    fn on_update(&mut self, gfx: &mut Gfx);
    
    fn on_resize(&mut self, gfx: &mut Gfx);
}

pub struct GfxContext<T>
    where T: GfxHandler + 'static 
{
    handler: T,
    gfx: Gfx,
    renderer: Renderer,
}


pub struct Gfx {
    pub canvas: Canvas,
    pub camera: Camera,
}

impl Gfx {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        
        let window = WindowBuilder::new().build(event_loop).unwrap();
        
        let canvas = Canvas::new(window).await;
        
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(), // position the camera one unit up and 2 units back, +z is out of the screen
            target: (0.0, 0.0, 0.0).into(), // have it look at the origin
            up: cgmath::Vector3::unit_y(), // which way is "up"
            aspect: canvas.aspect_ratio(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
        
        Gfx { canvas, camera }
    }
}

pub fn run<T> (mut handler: T) 
where T: GfxHandler + 'static 
{  
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut gfx = block_on(Gfx::new(&event_loop));
    let mut renderer = Renderer::new(&mut gfx);
    
    let mut ctx = GfxContext { handler, gfx, renderer };
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id, } => {
                if let Some(c) = window_event_handler(
                    &mut ctx,
                    event, 
                    &window_id
                ) {
                    *control_flow = c;
                }
            },
            Event::RedrawRequested(window_id) if window_id == ctx.gfx.canvas.window.id() => {
                update_handler(&mut ctx);
                
                if let Some(c) = draw_handler(&mut ctx) {
                    *control_flow = c;
                }
            },
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                ctx.gfx.canvas.window.request_redraw();
            },
            _ => {}
        }
    });
}

fn window_event_handler<T>(ctx: &mut GfxContext<T>, event: &WindowEvent, id: &WindowId)
-> Option<ControlFlow> where T: GfxHandler + 'static 
{

    if *id == ctx.gfx.canvas.window.id() && !ctx.handler.on_input(event) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => Some(ControlFlow::Exit),
            
            WindowEvent::Resized(physical_size) => {
                resize_handler(ctx, *physical_size);
                None
            },
            
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                resize_handler(ctx, **new_inner_size);
                None
            },
            _ => { None }
        }
    } else {
        None
    }
}

fn resize_handler<T> (ctx: &mut GfxContext<T>, new_size: winit::dpi::PhysicalSize<u32>)
where T: GfxHandler + 'static 
{
    
    if ctx.gfx.canvas.resize(new_size) {
        ctx.renderer.resize(&mut ctx.gfx, new_size);
        ctx.handler.on_resize(&mut ctx.gfx);
    }
}

fn update_handler<T> (ctx: &mut GfxContext<T>)
where T: GfxHandler + 'static 
{
    
    ctx.handler.on_update(&mut ctx.gfx);
    ctx.renderer.update(&mut ctx.gfx);
}

fn draw_handler<T>(ctx: &mut GfxContext<T>) -> Option<ControlFlow> 
where T: GfxHandler + 'static 
{
    
    match ctx.renderer.draw(&mut ctx.gfx) {
        Ok(_) => { None },
        
        // Reconfigure the surface if lost
        Err(wgpu::SurfaceError::Lost) => {
            ctx.gfx.canvas.reconfigure_surface();
            None
        },
        
        // The system is out of memory, we should probably quit
        Err(wgpu::SurfaceError::OutOfMemory) => {
            Some(ControlFlow::Exit)
        },
        // All other errors (Outdated, Timeout) should be resolved by the next frame
        Err(e) => {
            eprintln!("{:?}", e);
            None
        },
    }
}
