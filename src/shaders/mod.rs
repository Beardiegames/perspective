pub mod default;
pub mod lit;

pub type FragmentShader = String;
pub type VertexShader = String;

pub fn default_shaders() -> (FragmentShader, VertexShader) {
	(
		default::FRAGMENT.to_string(),
		default::VERTEX.to_string()
	)
}
