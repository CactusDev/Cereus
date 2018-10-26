
use cache::Cacheable;
use packet::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
	pub quote: Vec<Component>,
	pub id: usize,
	pub quoted: Option<String>,
	pub when: String,
	pub enabled: bool,
	pub token: String
}

impl Cacheable for Quote {

	fn name(&self) -> String {
		format!("{}:{}", &self.token, &self.id)
	}

	fn make_cacheable(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}
