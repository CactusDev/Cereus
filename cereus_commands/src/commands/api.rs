
use cereus_core::types::{Trust, Command, Quote, Social, Component, QuoteAddResponse};
use serde_json::{Value, from_value, json};

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

    pub fn get_random_quote(&self, channel: &str) -> Result<Quote, reqwest::Error> {
        let url = &format!("quote/{}/random", channel);
        get!(Quote, url, self.client, self.base)
    }

    pub fn get_quote(&self, channel: &str, id: &str) -> Result<Quote, reqwest::Error> {
        let url = &format!("quote/{}/{}", channel, id);
        get!(Quote, url, self.client, self.base)
    }

    pub fn create_quote(&self, channel: &str, quote: Vec<Component>) -> Result<QuoteAddResponse, reqwest::Error> {
        let url = &format!("quote/{}/create", channel);
        let body = json!({
            "response": quote
        });
        post!(QuoteAddResponse, url, body, self.client, self.base)
    }

    pub fn remove_quote(&self, channel: &str, id: &str) -> Result<(), reqwest::Error> {
        let url = &format!("quote/{}/{}", channel, id);
        delete!(url, self.client, self.base)
    }

    pub fn edit_quote(&self, channel: &str, id: &str, quote: Vec<Component>) -> Result<(), reqwest::Error> {
        let url = &format!("quote/{}/{}", channel, id);
        let body = json!({
            "response": quote
        });
        patch!(url, body, self.client, self.base)
    }

    pub fn get_command(&self, channel: &str, command: &str) -> Result<Command, reqwest::Error> {
        let url = &format!("command/{}/{}", channel, command);
        get!(Command, url, self.client, self.base)
    }

    pub fn create_command(&self, channel: &str, command: &str, response: Vec<Component>) -> Result<(), reqwest::Error> {
        let url = &format!("command/{}/{}", channel, command);
        let body = json!({
            "response": response,
            "services": json!([])  // TODO
        });
        match post!(Command, url, body, self.client, self.base) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    pub fn remove_command(&self, channel: &str, command: &str) -> Result<(), reqwest::Error> {
        let url = &format!("command/{}/{}", channel, command);
        match delete!(url, self.client, self.base) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    pub fn list_command(&self, channel: &str) -> Result<Vec<Command>, reqwest::Error> {
        let url = &format!("command/{}", channel);
        get!(Vec<Command>, url, self.client, self.base)
    }

    pub fn edit_command(&self, channel: &str, command: &str, response: Vec<Component>) -> Result<(), reqwest::Error> {
        let url = &format!("command/{}/{}", channel, command);
        let body = json!({
            "response": response,
            "services": json!([])
        });
        patch!(url, body, self.client, self.base)
    }

    pub fn get_trusts(&self, channel: &str) -> Result<Vec<Trust>, reqwest::Error> {
        let url = &format!("trust/{}", channel);
        get!(Vec<Trust>, url, self.client, self.base)
    }

    pub fn get_trust(&self, channel: &str, user: &str) -> Result<Trust, reqwest::Error> {
        let url = &format!("trust/{}/{}", channel, user);
        get!(Trust, url, self.client, self.base)
    }

    pub fn add_trust(&self, channel: &str, trusted: &str) -> Result<(), reqwest::Error> {
        let url = &format!("trust/{}/{}", channel, trusted);
        let body = json!({
        });
        post!((), url, body, self.client, self.base)
    }

    pub fn remove_trust(&self, channel: &str, trusted: &str) -> Result<(), reqwest::Error> {
        let url = &format!("trust/{}/{}", channel, trusted);
        delete!(url, self.client, self.base)
    }

    pub fn get_socials(&self, channel: &str) -> Result<Vec<Social>, reqwest::Error> {
        let url = &format!("socials/{}", channel);
        get!(Vec<Social>, url, self.client, self.base)
    }

    pub fn get_social(&self, channel: &str, service: &str) -> Result<Social, reqwest::Error> {
        let url = &format!("socials/{}/{}", channel, service);
        get!(Social, url, self.client, self.base)
    }
}
