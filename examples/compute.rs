use perspective::*;


pub struct ComputeExample {
    compute: ComputeProcessor,
}


impl PerspectiveHandler for ComputeExample {

    fn startup(gfx: &mut WgpuCore) -> Self {

        let compute = gfx.setup_compute_processor(
            &ComputeSettings {
                label: "ComputeExample", 
                group_index: 0,// represented within shader as @binding
                binding_index: 0,// represented within shader as @binding
    
                data_set: vec![1; 10000],
                shader_src: include_str!("shaders/compute_shader.wgsl"),
                entry_point: "main",
            }
        );

        ComputeExample { compute }
    }

    fn render_pipeline(&mut self, mut ctx: RenderContext) {
        self.compute.quick_inject_passes(&mut ctx.encoder);

        ctx.gx.queue.submit(Some(ctx.encoder.finish()));
        
        let buffer = self.compute.slice_staging_buffer();

        ctx.gx.device.poll(wgpu::Maintain::Wait); 
        
        let data = buffer.get_mapped_range();

        let result: Vec<u32> = data
            .chunks_exact(self.compute.buffer_chunksize)
            .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
            .collect();

        drop(data);
        self.compute.staging_buffer.unmap();

        println!("result_data: {}, {}, {}, {}", result[0], result[1], result[2], result[3]);
    }
}

fn main() -> anyhow::Result<()> {

    let window_size = PhysicalSize::new(1600, 1200);
    let camera_setup = CameraSetup::default();

    Perspective::new(window_size, camera_setup)
        .run::<ComputeExample>()
}



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
