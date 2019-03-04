
use packet::Packet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
	pub quote_id: i64,
	pub response: Vec<Packet>,
	pub channel: String
}
