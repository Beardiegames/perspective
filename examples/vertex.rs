use perspective::*;


pub struct VertexExample {
}


impl PerspectiveHandler for VertexExample {

    fn startup(_gfx: &mut WgpuCore) -> Self {

        let shader = include_str!("shaders/basic_shader.wgsl");

        VertexExample {  }
    }
}

fn main() -> anyhow::Result<()> {

    Perspective::new(800, 600)
        .run::<VertexExample>()?;
    
    Ok(())    
}