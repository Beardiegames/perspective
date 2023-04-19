mod gfx;

use winit::{
	event_loop::EventLoop, 
	window::WindowBuilder
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
    let window = &WindowBuilder::new().build(&event_loop).unwrap();

    // create wgpu handle
    let mut wgpu_core = gfx::WgpuCore::new(Some(
        gfx::WindowSettings { window, width: 800, height: 600 } 
    ))?;

    // log to check if everything is going well
    println!("{:?}", wgpu_core.adapter.get_info());

    // setup compute data
    let data = vec![1, 2, 3, 4];
    
    // setup compute shader handles
    let compute_processor = gfx::ComputeProcessor::new(
        &mut wgpu_core, 
        include_str!("shaders/compute_shader.wgsl"), 
        data
    );

    // build compute shader
    let cbuff = compute_processor.execute(&wgpu_core);

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    wgpu_core.device.poll(wgpu::Maintain::Wait); 

    let udat = compute_processor.post_render(cbuff);

    for val in &udat {
        println!("val: {}", val);
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
