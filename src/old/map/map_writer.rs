use std::{fs};
use serde::Serialize;
use toml::*;
use crate::*;


impl Map {
	pub fn write_to_file(&self, file_path: &str) -> Result<(), MapError> {
			let toml = toml::to_string(self).unwrap();

			fs::write(file_path, toml)
				.map_err(|_x| MapError::WriteFileFailed)?;
				
			Ok(())
	}
}
