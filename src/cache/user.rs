
use cache::Cacheable;

#[derive(Serialize, Deserialize, Clone)]
pub struct Repeat {
	pub disabled: bool,
	pub only_live: bool,
	pub default_minimum: u32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EventConfig {
	pub message: String,  // TODO: This should probably be components
	pub enabled: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Events {
	pub follow:    EventConfig,
	pub subscribe: EventConfig,
	pub host:      EventConfig,
	pub join:      EventConfig,
	pub leave:     EventConfig
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SpamAction {
	Ignore,
	Purge,
	Timeout,
	Ban
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpamConfig<T> {
	pub action: SpamAction,
	pub value: T,
	pub warnings: u16
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Keywords {
	pub blacklist: Vec<String>,
	pub whitelist: Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NotifyMod {
	pub notify: bool
	// TODO: Maybe we should use the user's role here?
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Spam {
	pub allow_urls:     SpamConfig<bool>,
	pub max_caps_score: SpamConfig<u32>,
	pub max_emoji:      SpamConfig<u32>,
	pub keywords:       Keywords,
	pub notify_mod:     NotifyMod
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
	pub repeat: Repeat,
	pub events: Events,
	pub whitelisted_urls: Vec<String>,
	pub spam: Spam
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceAuthentication {
	pub service: String,
	pub account_name: String,
	pub auth_key: String,
	pub refresh: Option<String>,
	pub expires: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Service {
	pub name: String,
	pub api_base: String  // TODO: this should be converted to a really cool datastructure
	                      // that we can use for "dynamic" api things.
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
	pub service: Service,
	pub authentication: ServiceAuthentication,
	pub token: String,
	pub user: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
	pub channels: Vec<Channel>,
	pub config: Config,
	pub token: String,
	pub id: String
}

impl Cacheable for User {

	fn name(&self) -> String {
		self.id.clone()
	}

	fn make_cacheable(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}
