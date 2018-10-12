
use packet::*;

#[macro_export]
macro_rules! command {
	($($key:expr => $value:expr),+) => {
		{
			let mut test: std::collections::HashMap<&str, Box<Fn(String) -> $crate::packet::Packet>> = std::collections::HashMap::new();
			$(
				test.insert($key, Box::new($value));
			)+
			test
		}
	}
}
