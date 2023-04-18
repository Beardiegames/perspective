
pub const FRAGMENT: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
}
";

pub const VERTEX: &'static str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";
