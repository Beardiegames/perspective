use std::{fs};
use serde::Deserialize;
use toml::*;
use crate::*;


impl Map {
	pub fn read_from_file(file_path: &str) -> Result<Map, MapError> {
			let toml = fs::read_to_string(file_path)
				.map_err(|x| MapError::ReadFileFailed)?;

			let map: Map = toml::from_str(&toml)
				.map_err(|x| MapError::ParseFileFailed)?;

			Ok(map)
	}
}
