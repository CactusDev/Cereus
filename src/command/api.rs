
use types::Quote;

pub struct CommandAPI {
	client: reqwest::Client,
	base: String
}

impl CommandAPI {

	pub fn new(base: &str) -> Self {
		CommandAPI {
            client: reqwest::Client::new(),
            base: base.to_string()
		}
	}

	pub fn get_api_url(&self, endpoint: &str) -> String {
		format!("{}/{}", &self.base, endpoint)
	}

	pub fn get_random_quote(&self, channel: &str) -> Result<Quote, reqwest::Error> {
		let url = self.get_api_url(&format!("quote/{}/random", channel));
		self.client.get(&url).send()?.error_for_status()?.json()
	}

	pub fn get_quote(&self, channel: &str, id: u32) -> Result<Quote, reqwest::Error> {
		let url = self.get_api_url(&format!("quote/{}/{}", channel, id));
		self.client.get(&url).send()?.error_for_status()?.json()
	}
}
