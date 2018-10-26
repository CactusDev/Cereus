
use cache::Cacheable;
use packet::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
	pub name: String,
	pub response: Vec<Component>,
	pub count: usize,
	pub enabled: bool,
	pub token: String
}

impl Cacheable for Command {

	fn name(&self) -> String {
		format!("{}:{}", &self.token, &self.name)
	}

	fn make_cacheable(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}
