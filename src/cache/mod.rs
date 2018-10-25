
use std::collections::HashMap;

/// Describes an item as something that can be cached.
pub trait Cacheable {
	fn make_cacheable(&self);
}

pub struct Cache {
	cache_time: isize,
	prefix: String
}

impl Cache {

	pub fn new(cache_time: isize, cache_name: &str) -> Self {
		// First, create the cacheable name for redis
		let prefix = format!("cache-{}", cache_name);
		
		Cache {
			cache_time,
			prefix
		}
	}

	fn get_cacheable_name(&self, name: &str) -> String {
		format!("{}-{}", &self.prefix, name)
	}
}
