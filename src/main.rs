mod gfx;
use gfx::encoders::FromBytes;

use winit::{
	event_loop::EventLoop, 
	window::WindowBuilder
};

impl FromBytes for u32 {
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
    let window = &WindowBuilder::new().build(&event_loop).unwrap();

    // create wgpu handle
    let mut wgpu_core = gfx::WgpuCore::new(Some(
        gfx::WindowSettings { window, width: 800, height: 600 } 
    ))?;

    // log to check if everything is going well
    println!("{:?}", wgpu_core.adapter.get_info());

    // setup compute data
    let vec = vec![1, 2, 3, 4];
    let data = &vec.as_slice();
    
    // setup compute shader handles
    let shader_str = include_str!("shaders/compute_shader.wgsl");
    let compute_pipe = wgpu_core.setup_compute_pipeline("compute_pipe", shader_str, "main");
    let compute_buffers = wgpu_core.create_buffer_handles("compute_buffer", data);
    let compute_bindgroup = wgpu_core.setup_bind_group("compute_group", &compute_pipe, &compute_buffers);

    // build compute shader
    let compute_process = wgpu_core.execute_compute_passes(&compute_pipe, &compute_bindgroup, &compute_buffers);

     	// Poll the device in a blocking manner so that our future resolves.
     // In an actual application, `device.poll(...)` should
     // be called in an event loop or on another thread.
     wgpu_core.device.poll(wgpu::Maintain::Wait); 

	match compute_process.state() {
		std::task::Poll::Ready(r) => {
			match r {
				Ok(data) => println!("DATA: {:?}", data),
				Err(e) => println!("ERROR: {}", e.to_string()),
			};
		},
		std::task::Poll::Pending => {
			println!("STILL PENDING!!!");
		}
	}
	
    
    // // run
    // event_loop.run(move |event, _, control_flow| match event {
    //     Event::WindowEvent {
    //         ref event,
    //         window_id,
    //     } if window_id == window.id() => match event {
    //         WindowEvent::CloseRequested
    //         | WindowEvent::KeyboardInput {
    //             input:
    //                 KeyboardInput {
    //                     state: ElementState::Pressed,
    //                     virtual_keycode: Some(VirtualKeyCode::Escape),
    //                     ..
    //                 },
    //             ..
    //         } => *control_flow = ControlFlow::Exit,
    //         _ => {}
    //     },
    //     _ => {}
    // });

    Ok(())    
}
