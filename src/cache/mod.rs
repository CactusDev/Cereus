
use redis;

use config::RedisConfig;

/// Describes an item as something that can be cached.
pub trait Cacheable {
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
		let prefix = format!("cache-{}", cache_name);

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
		format!("{}-{}", &self.prefix, name)
	}

	fn cache(&mut self, object: Box<Cacheable>) -> Result<(), redis::RedisError> {
		let stringified = object.make_cacheable();
		println!("{}", &stringified);
		// Now, we need to toss this into Redis.
		Ok(())
	}
}
