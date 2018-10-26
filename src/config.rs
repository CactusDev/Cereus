
use std::{
	fs::File,
	io::prelude::*
};

pub enum ConfigurationError {
	FileNotFound,
	CouldNotRead,
	CouldNotParse
}

#[derive(Deserialize)]
pub struct RedisConfig {
	pub host: String,
	pub port: i16,
	pub password: Option<String>,
	pub db: usize,
}

#[derive(Deserialize)]
pub struct CereusConfiguration {
	pub port: i16,
	pub redis: RedisConfig
}

impl CereusConfiguration {

	pub fn new(path: &str) -> Result<Self, ConfigurationError> {
		let mut file = File::open(path).map_err(|_| ConfigurationError::FileNotFound)?;
		let mut contents = String::new();

		file.read_to_string(&mut contents).map_err(|_| ConfigurationError::CouldNotRead)?;
		serde_json::from_str::<CereusConfiguration>(&contents).map_err(|_| ConfigurationError::CouldNotParse)
	}
}
