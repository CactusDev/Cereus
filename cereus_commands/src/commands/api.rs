
use crate::commands::{APIHandler, APIResult};
use cereus_core::types::{Trust, Command, Quote, Social, Component, QuoteAddResponse, ChangeCommandStateResponse, UpdateCountResult};
use serde_json::{Value, from_value, json};

pub struct CactusAPI {
    client: reqwest::Client,
    base: String
}

impl CactusAPI {
    pub fn new(base: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base: base.to_string()
        }
    }
}

impl APIHandler for CactusAPI {

    fn get_random_quote(&self, channel: &str) -> APIResult<Quote> {
        let url = &format!("quote/{}/random", channel);
        get!(Quote, url, self.client, self.base)
    }

    fn get_quote(&self, channel: &str, id: &str) -> APIResult<Quote> {
        let url = &format!("quote/{}/{}", channel, id);
        get!(Quote, url, self.client, self.base)
    }

    fn create_quote(&self, channel: &str, quote: Vec<Component>) -> APIResult<QuoteAddResponse> {
        let url = &format!("quote/{}/create", channel);
        let body = json!({
            "response": quote
        });
        post!(QuoteAddResponse, url, body, self.client, self.base)
    }

    fn remove_quote(&self, channel: &str, id: &str) -> APIResult<()> {
        let url = &format!("quote/{}/{}", channel, id);
        delete!(url, self.client, self.base)
    }

    fn edit_quote(&self, channel: &str, id: &str, quote: Vec<Component>) -> APIResult<()> {
        let url = &format!("quote/{}/{}", channel, id);
        let body = json!({
            "response": quote
        });
        patch!(url, body, self.client, self.base)
    }

    fn get_command(&self, channel: &str, command: &str) -> APIResult<Command> {
        let url = &format!("command/{}/{}", channel, command);
        get!(Command, url, self.client, self.base)
    }

    fn create_command(&self, channel: &str, command: &str, response: Vec<Component>, role: &str) -> APIResult<()> {
        let url = &format!("command/{}/{}", channel, command);
        let body = json!({
            "response": response,
            "services": json!([]),  // TODO
            "role": role
        });
        match post!(Command, url, body, self.client, self.base) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn remove_command(&self, channel: &str, command: &str) -> APIResult<()> {
        let url = &format!("command/{}/{}", channel, command);
        match delete!(url, self.client, self.base) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn list_command(&self, channel: &str) -> APIResult<Vec<Command>> {
        let url = &format!("command/{}", channel);
        get!(Vec<Command>, url, self.client, self.base)
    }

    fn edit_command(&self, channel: &str, command: &str, response: Vec<Component>) -> APIResult<()> {
        let url = &format!("command/{}/{}", channel, command);
        let body = json!({
            "response": response,
            "services": json!([])
        });
        patch!(url, body, self.client, self.base)
    }

    fn get_trusts(&self, channel: &str) -> APIResult<Vec<Trust>> {
        let url = &format!("trust/{}", channel);
        get!(Vec<Trust>, url, self.client, self.base)
    }

    fn get_trust(&self, channel: &str, user: &str) -> APIResult<Trust> {
        let url = &format!("trust/{}/{}", channel, user);
        get!(Trust, url, self.client, self.base)
    }

    fn add_trust(&self, channel: &str, trusted: &str) -> APIResult<()> {
        let url = &format!("trust/{}/{}", channel, trusted);
        let body = json!({
            // TODO
        });
        post!((), url, body, self.client, self.base)
    }

    fn remove_trust(&self, channel: &str, trusted: &str) -> APIResult<()> {
        let url = &format!("trust/{}/{}", channel, trusted);
        delete!(url, self.client, self.base)
    }

    fn get_socials(&self, channel: &str) -> APIResult<Vec<Social>> {
        let url = &format!("social/{}", channel);
        get!(Vec<Social>, url, self.client, self.base)
    }

    fn get_social(&self, channel: &str, service: &str) -> APIResult<Social> {
        let url = &format!("social/{}/{}", channel, service);
        get!(Social, url, self.client, self.base)
    }

    fn add_social(&self, channel: &str, service: &str, address: &str) -> APIResult<()> {
        let url = &format!("social/{}/{}", channel, service);
        
        let body = json!({
            "url": address
        });

        patch!(url, body, self.client, self.base)
    }

    fn remove_social(&self, channel: &str, service: &str) -> APIResult<()> {
        let url = &format!("social/{}/{}", channel, service);
        delete!(url, self.client, self.base)
    }

    fn get_offences(&self, channel: &str, service: &str, user: &str, ty: &str) ->  APIResult<i32> {
        let url = &format!("offences/{}/{}/{}/{}", channel, service, user, ty);
        get!(i32, url, self.client, self.base)
    }

    fn update_user_offences(&self, channel: &str, service: &str, user: &str, ty: &str, operation: &str, amount: &str) ->  APIResult<()> {
        let url = &format!("offences/{}/{}/{}/{}", channel, service, user, ty);

        let body = json!({
            "count": format!("{}{}", operation, amount)  // TODO: probably slow and bad
        });

        patch!(url, body, self.client, self.base)
    }

    fn change_command_state(&self, channel: &str, command: &str, state: bool) -> APIResult<ChangeCommandStateResponse> {
        let url = &format!("command/{}/{}/state", channel, command);

        let body = json!({
            "state": state
        });

        patch!(ChangeCommandStateResponse, url, body, self.client, self.base)
    }

    fn update_count(&self, channel: &str, command: &str, count: &str) -> APIResult<UpdateCountResult> {
        let url = &format!("command/{}/{}/count", channel, command);

        let body = json!({
            "count": count
        });

        patch!(UpdateCountResult, url, body, self.client, self.base)
    }
}
