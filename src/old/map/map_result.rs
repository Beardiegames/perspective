
#[derive(Debug)]
pub enum MapError {
	GenericError,
	ReadFileFailed,
	ParseFileFailed,
	WriteFileFailed,

	OutOfBounds(String),
}

impl MapError {
	pub fn out_of_bounds(param: &str, value: &str, expected: &str) -> MapError {
		MapError::OutOfBounds(
			format!("Parameter '{}' has a value of '{}' which is too large! Value may not exceed {}", 
				param, 
				value, 
				expected
			)
		)
	}
}
