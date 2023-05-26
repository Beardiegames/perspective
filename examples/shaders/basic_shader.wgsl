// Vertex shader
// -------------

struct CameraUniform {
    projection_matrix:  mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) col: vec3<f32>,
    @location(2) uv_map: vec2<f32>,
    @location(3) uv_scale: vec2<f32>,
    //@location(4) uv_offset: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) index: u32,
    @location(1) uv: vec2<f32>,
};

struct InstanceInput {
    @location(5) index: u32,

    @location(6) model_matrix_0: vec4<f32>,
    @location(7) model_matrix_1: vec4<f32>,
    @location(8) model_matrix_2: vec4<f32>,
    @location(9) model_matrix_3: vec4<f32>,
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
    out.index = instance.index;
    out.uv = model.uv_map * model.uv_scale;
    
    out.clip_position = camera.projection_matrix * model_matrix * vec4<f32>(model.pos, 1.0);
    return out;
}


// Fragment shader
// ---------------

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@group(2) @binding(0) 
var<storage> sprite_frames: array<vec2<f32>>;

@group(2) @binding(1) 
var<uniform> frames_passed: u32;

struct SpriteAnimationData {
    frames: vec2<u32>,
    offset: u32,
    head: u32,
};
@group(2) @binding(2) 
var<storage, read_write> animations: array<SpriteAnimationData>;


@fragment
fn fragment_main(
    in: VertexOutput
) 
    -> @location(0) vec4<f32> 
{
    let uv = in.uv + sprite_animation(in.index);
    return textureSample(t_diffuse, s_diffuse, uv);
}

fn sprite_animation(i: u32) -> vec2<f32> {
    var head: u32 = animations[i].head;
    var start = animations[i].frames.x;
    var end = animations[i].frames.y;
    var offset = animations[i].offset;

    if head < start {
        head = start;
    }

    var len: u32 = 1u + end - start;
    head = (frames_passed + offset) % len;

    if head >= arrayLength(&sprite_frames) {
        head = 0u;
    }

    animations[i].head = head;
    return sprite_frames[head];
}
