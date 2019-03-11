
use types::{Command, Quote};
use packet::{Component};
use serde_json::{Value, from_value, json};

pub struct CommandAPI {
	client: reqwest::Client,
	base: String
}

#[derive(Deserialize)]
pub struct QuoteAddResponse {
	pub created: bool,
	pub id: u32
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
		let result: Value = self.client.get(&url)
			.send()?.error_for_status()?.json()?;
		Ok(from_value(result["data"].clone()).unwrap())
	}

	pub fn get_quote(&self, channel: &str, id: &str) -> Result<Quote, reqwest::Error> {
		let url = self.get_api_url(&format!("quote/{}/{}", channel, id));
		let result: Value = self.client.get(&url)
			.send()?.error_for_status()?.json()?;
		Ok(from_value(result["data"].clone()).unwrap())
	}

	pub fn create_quote(&self, channel: &str, quote: Vec<Component>) -> Result<QuoteAddResponse, reqwest::Error> {
		let url = self.get_api_url(&format!("quote/{}/create", channel));
		let body = json!({
			"response": quote
		});
		let result: Value = self.client.post(&url)
			.json(&body)
			.send()?.error_for_status()?.json()?;
		Ok(from_value(result["data"].clone()).unwrap())
	}

	pub fn get_command(&self, channel: &str, command: &str) -> Result<Command, reqwest::Error> {
		let url = self.get_api_url(&format!("command/{}/{}", channel, command));
		let thing: Value = self.client.get(&url).send()?.error_for_status()?.json()?;
		Ok(from_value(thing["data"].clone()).unwrap())
	}

	pub fn create_command(&self, channel: &str, command: &str, response: Vec<Component>) -> Result<(), reqwest::Error> {
		let url = self.get_api_url(&format!("command/{}/{}", channel, command));
		let body = json!({
			"response": response,
			"services": json!([])
		});
		self.client.post(&url)
			.json(&body)
			.send()?.error_for_status()?;
		Ok(())
	}

	pub fn remove_command(&self, channel: &str, command: &str) -> Result<(), reqwest::Error> {
		let url = self.get_api_url(&format!("command/{}/{}", channel, command));
		self.client.delete(&url)
			.send()?.error_for_status()?;
		Ok(())
	}

	pub fn list_command(&self, channel: &str) -> Result<Vec<Command>, reqwest::Error> {
		let url = self.get_api_url(&format!("command/{}", channel));
		let thing: Value = self.client.get(&url).send()?.error_for_status()?.json()?;
		Ok(from_value(thing["data"].clone()).unwrap())
	}

	pub fn edit_command(&self, channel: &str, command: &str, response: Vec<Component>) -> Result<(), reqwest::Error> {
		let url = self.get_api_url(&format!("command/{}/{}", channel, command));
		let body = json!({
			"response": response,
			"services": json!([])
		});
		self.client.patch(&url).json(&body)
			.send()?.error_for_status()?;
		Ok(())
	}
}
