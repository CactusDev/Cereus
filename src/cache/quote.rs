
use packet::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
	pub quote: Vec<Component>,
	pub id: usize,
	pub quoted: Option<String>,
	pub when: String,
	pub enabled: bool
}
