// Vertex shader

struct UniformData {
    time: u32,
    projection_matrix:  mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> global: UniformData;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) col: vec3<f32>,
    @location(2) uv_map: vec2<f32>,
    @location(3) uv_scale: vec2<f32>,
    //@location(4) uv_offset: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,

    @location(8) frame: u32,
};


@vertex
fn vertex_main(
    model: VertexInput,
    instance: InstanceInput,
) 
    -> VertexOutput 
{
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    
    var out: VertexOutput;
    out.uv = model.uv_map * model.uv_scale;

    var offset: vec2<f32>;
    let secs = fract(global.time / 250000);
    let frame = floor(secs / 250000.0);
    let unbound_col = (model.uv_scale[0] * frame);

    offset[1] = (floor(unbound_col) * model.uv_scale[1]);
    offset[0] = unbound_col % 1.0;
    out.uv += offset;

    out.clip_position = global.projection_matrix * model_matrix * vec4<f32>(model.pos, 1.0);
    return out;
}


// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fragment_main(
    in: VertexOutput
) 
    -> @location(0) vec4<f32> 
{
    return textureSample(t_diffuse, s_diffuse, in.uv);
}
