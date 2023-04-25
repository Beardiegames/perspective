use perspective::*;


pub struct ComputeExample {
    compute_processor: ComputeProcessor,
}


impl PerspectiveHandler for ComputeExample {

    fn startup(gfx: &mut WgpuCore) -> Self {

        let compute_processor = ComputeProcessor::new(
            gfx, 
            &ComputeSettings {
                label: "Example", 
                group_index: 0,// represented within shader as @binding
                binding_index: 0,// represented within shader as @binding
    
                start_data: vec![1; 10000],
                shader_src: include_str!("shaders/compute_shader.wgsl"),
                entry_point: "main",
            }
        );

        ComputeExample { compute_processor }
    }

    fn render_pipeline(&mut self, gx: &WgpuCore, mut encoder: wgpu::CommandEncoder) -> Result<(), wgpu::SurfaceError> {
        self.compute_processor.inject_passes(&mut encoder);

        gx.queue.submit(std::iter::once(encoder.finish()));
        
        let buffer = self.compute_processor.slice_staging_buffer();

        gx.device.poll(wgpu::Maintain::Wait); 
        
        let result_data = self.compute_processor.read_results_and_drop(
            buffer,
            |b| u32::from_ne_bytes(b.try_into().unwrap()),
        );

        println!("result_data: {}, {}, {}, {}", result_data[0], result_data[1], result_data[2], result_data[3]);

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {

    Perspective::new(800, 600)
        .run::<ComputeExample>()?;
    
    Ok(())    
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
