
use packet::{Packet, Component};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
	pub quote_id: i64,
	pub response: Vec<Packet>,
	pub channel: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandMeta {
	pub added_by: String,
	pub cooldown: i32,
	pub count: i32,
	pub enabled: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
	pub channel: String,
	pub created_at: String,
	pub deleted_at: Option<String>,
	pub meta: CommandMeta,
	pub name: String,
	pub response: Vec<Component>,
	pub services: Vec<String>,
	pub updated_at: String
}
