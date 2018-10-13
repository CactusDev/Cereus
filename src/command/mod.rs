
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

#[macro_export]
macro_rules! url {
	($url:expr) => {
		Component::URL($url.to_string())
	}
}

#[macro_export]
macro_rules! text {
	($text:expr) => {
		Component::Text($text.to_string())
	}
}

#[macro_export]
macro_rules! emoji {
	($emoji:expr) => {
		Component::Emoji($emoji.to_string())
	}
}

#[macro_export]
macro_rules! tag {
	($tag:expr) => {
		Component::Tag($tag.to_string())
	}
}
