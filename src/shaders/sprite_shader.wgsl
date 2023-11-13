// Vertex shader
// -------------

struct CameraUniform {
    projection_matrix:  mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct AmbientLight {
    color: vec4<f32>,
}
@group(2) @binding(0)
var<uniform> ambient: AmbientLight;

struct PointLight {
    position: vec4<f32>,
    color: vec4<f32>,
    //range: f32,
}
@group(2) @binding(1) 
var<storage> point_lights: array<PointLight>;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) col: vec3<f32>,
    @location(2) uv_map: vec2<f32>,
    @location(3) uv_scale: vec2<f32>,
    @location(4) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) index: u32,
    @location(1) uv: vec2<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
};

struct InstanceInput {
    @location(5) index: u32,
    @location(6) model_matrix_0: vec4<f32>,
    @location(7) model_matrix_1: vec4<f32>,
    @location(8) model_matrix_2: vec4<f32>,
    @location(9) model_matrix_3: vec4<f32>,
};

@vertex
fn vert(
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
    out.world_normal = model.normal;
    
    var world_position: vec4<f32> = model_matrix * vec4<f32>(model.pos, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.projection_matrix * world_position;
    return out;
}


// Fragment shader
// ---------------

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@group(3) @binding(0) 
var<storage> sprite_frames: array<vec2<f32>>;

@group(3) @binding(1) 
var<uniform> frames_passed: u32;

struct SpriteAnimationData {
    frames: vec2<u32>,
    offset: u32,
    head: u32,
};
@group(3) @binding(2) 
var<storage, read_write> animations: array<SpriteAnimationData>;


@fragment
fn frag(
    in: VertexOutput
) 
    -> @location(0) vec4<f32> 
{
    let uv = in.uv + sprite_animation(in.index);
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, uv);

    var lit_color = object_color.xyz * ambient.color.xyz;

    let num_lights = arrayLength(&point_lights);
    for(var i: u32 = 0u; i < num_lights; i=i+1u) {
    //    //let dist = distance(center, li);
        let light_strength = max(dot(in.world_normal, point_lights[i].position.xyz), 0.0);
        lit_color = mix(lit_color, point_lights[i].color.xyz, light_strength);
    }
    
    //let light_strength = max(dot(in.world_normal, ambient_light.direction), 0.0);
    //let shadow_strength = 1.0 - light_strength;
    //let light_color = ambient_light.light_color * light_strength;
    //let shadow_color = ambient_light.shadow_color * shadow_strength;
    //let result = (light_color + shadow_color) * object_color.xyz;
    
    
    return vec4<f32>(lit_color, object_color.a);
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
