mod gfx;

fn main() -> anyhow::Result<()> {

    let gfx_core = gfx::WgpuCore::new(None)?;
    println!("{:?}", gfx_core.adapter.get_info());

    Ok(())
}
