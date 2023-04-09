mod gfx;

fn main() -> anyhow::Result<()> {

    let vec = vec![1, 2, 3, 4];
    let data = &vec.as_slice();
    
    let mut wgpu_core = gfx::WgpuCore::new(None)?;
    println!("{:?}", wgpu_core.adapter.get_info());

    let shader_str = include_str!("shaders/compute_shader.wgsl");
    let compute_pipe = wgpu_core.setup_compute_pipeline("compute_pipe", shader_str, "main");
    let compute_buffers = wgpu_core.create_buffer_handles("compute_buffer", data);

    let compute_bindgroup = wgpu_core.setup_bind_group("compute_group", &compute_pipe, &compute_buffers);

    wgpu_core.setup_compute_command_encoder(&compute_pipe, &compute_bindgroup, &compute_buffers);

    Ok(())
}
