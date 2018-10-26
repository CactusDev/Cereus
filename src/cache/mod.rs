
use redis;

/// Describes an item as something that can be cached.
pub trait Cacheable {
	fn make_cacheable(&self) -> String;
}

pub struct Cache {
	cache_time: isize,
	prefix:     String,

	redis_password:   String,
	redis_ip:         String,
	redis_port:       u16,
	redis_db:         i64,
	redis_connection: Option<redis::Client>
}

impl Cache {

	pub fn new(cache_time: isize, cache_name: &str, ip: &str, port: u16, db: i64, password: &str) -> Self {
		let prefix = format!("cache-{}", cache_name);
		
		Cache {
			cache_time,
			prefix,

			redis_password: password.to_string(),
			redis_ip: ip.to_string(),
			redis_port: port,
			redis_db: db,
			redis_connection: None
		}
	}

	fn connect_to_redis(&mut self) -> Result<(), redis::RedisError> {
		let addr = redis::ConnectionAddr::Tcp(self.redis_ip.clone(), self.redis_port);
		let connection_info = redis::ConnectionInfo {
			addr: Box::new(addr),
			db: self.redis_db,
			passwd: Some(self.redis_password.clone())
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
