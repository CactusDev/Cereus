
use packet::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
	pub name: String,
	pub response: Vec<Component>,
	pub count: usize,
	pub enabled: bool
}
