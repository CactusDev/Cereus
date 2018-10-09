
use iron::{
	prelude::*,
	Listening,
	status,
	Iron,
	method::Method
};

use packet::*;
use serde_json;
use std::io::Read;

pub struct WebServer {
	addr: String,
	port: u32,
	server: Option<Listening>
}

impl WebServer {

	pub fn new(addr: &str, port: u32) -> Self {
		WebServer {
			addr: addr.to_string(),
			port,
			server: None
		}
	}

	pub fn listen(&mut self) {
		let host = format!("{}:{}", &self.addr, self.port);

		self.server = Some(Iron::new(|req: &mut Request| {
			let path = req.url.path();
			let first = path[0];
			if first != "response" {
				return Ok(Response::with((status::NotFound, "invalid endpoint")))
			}

			if req.method != Method::Get {
				return Ok(Response::with((status::MethodNotAllowed, "Only get is allowed")))
			}
			// Since this is the response endpoint, we'll actually try to do something with the data.
			let mut data = String::new();
			match req.body.read_to_string(&mut data) {
				Ok(_) => {
					// Attempt to load the data as json
					match serde_json::from_str::<Context>(&data) {
						Ok(context) => Ok(Response::with((status::Ok, "worked"))),
						Err(_) => Ok(Response::with((status::BadRequest, "invalid json")))
					}
				},
				Err(_) => Ok(Response::with((status::BadRequest, "invalid data provided")))
			}
		}).http(&host).unwrap());
		println!("Listening on {}", &host);
	}
}
