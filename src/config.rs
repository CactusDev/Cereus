
use std::{
	fs::File,
	io::prelude::*
};

pub enum ConfigurationError {
	FileNotFound,
	CouldNotRead,
	CouldNotParse
}

impl std::fmt::Display for ConfigurationError {

	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			&ConfigurationError::FileNotFound => write!(f, "Could not find configuration file"),
			&ConfigurationError::CouldNotRead => write!(f, "Could not read contents of configuration file"),
			&ConfigurationError::CouldNotParse => write!(f, "Could not parse configuration into json")
		}
	}
}

#[derive(Deserialize, Clone)]
pub struct RedisConfig {
	pub host: String,
	pub port: u16,
	pub password: Option<String>,
	pub db: i64,
}

#[derive(Deserialize, Clone)]
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
