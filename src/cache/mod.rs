
use redis;
use redis::Commands;

use config::RedisConfig;

/// Describes an item as something that can be cached.
pub trait Cacheable {
	fn name(&self) -> String;
	fn make_cacheable(&self) -> String;
}

pub struct Cache {
	cache_time: isize,
	prefix:     String,

	redis_config: RedisConfig,
	redis_connection: Option<redis::Client>
}

impl Cache {

	pub fn new(cache_time: isize, cache_name: &str, redis_config: &RedisConfig) -> Self {
		let prefix = format!("cache:{}", cache_name);

		Cache {
			cache_time,
			prefix,

			redis_config: redis_config.clone(),
			redis_connection: None
		}
	}

	fn connect_to_redis(&mut self) -> Result<(), redis::RedisError> {
		let addr = redis::ConnectionAddr::Tcp(self.redis_config.clone().host, self.redis_config.port);
		let connection_info = redis::ConnectionInfo {
			addr: Box::new(addr),
			db: self.redis_config.db,
			passwd: self.redis_config.clone().password
		};
		let connection = redis::Client::open(connection_info)?;

		self.redis_connection = Some(connection);

		Ok(())
	}

	fn get_cacheable_name(&self, name: &str) -> String {
		format!("{}:{}", &self.prefix, name)
	}

	pub fn cache(&mut self, object: Box<Cacheable>) -> Result<(), redis::RedisError> {
		if let None = self.redis_connection {
			// Connect to redis if we're not currently
			self.connect_to_redis()?;
		}
		// Get the cacheable name for this
		let name = self.get_cacheable_name(&object.name());
		let stringified = object.make_cacheable();

		match &self.redis_connection {
			Some(ref connection) => connection.set(name, stringified)?,
			None => {}
		}
		Ok(())
	}

	pub fn get(&mut self, name: String) -> Result<String, String> {
		if let None = self.redis_connection {
			// Connect to redis if we're not currently
			self.connect_to_redis().map_err(|err| err.category().to_string())?;
		}
		// Get the cacheable name for this
		let name = self.get_cacheable_name(&name);

		match &self.redis_connection {
			Some(ref connection) => connection.get(name).map_err(|err| err.category().to_string()),
			None => Err("could not get redis connection".to_string())
		}
	}
}
