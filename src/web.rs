
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

use handler::HandlerHandler;

pub struct WebServer {
	addr: String,
	port: u32,
	server: Option<Listening>,
	handler: HandlerHandler
}

impl WebServer {

	pub fn new(addr: &str, port: u32, handler: HandlerHandler) -> Self {
		WebServer {
			addr: addr.to_string(),
			port,
			server: None,
			handler
		}
	}

	pub fn listen(mut self) {
		let host = format!("{}:{}", &self.addr, self.port);

		let handler = self.handler;
		self.server = Some(Iron::new(move |request: &mut Request| {
			let path = request.url.path();
			let first = path[0];
			if first != "response" {
				return Ok(Response::with((status::NotFound, "invalid endpoint")))
			}

			if request.method != Method::Get {
				return Ok(Response::with((status::MethodNotAllowed, "Only get is allowed")))
			}
			// Since this is the response endpoint, we'll actually try to do something with the data.
			let mut data = String::new();
			match request.body.read_to_string(&mut data) {
				Ok(_) => {
					// Attempt to load the data as json
					match serde_json::from_str::<Context>(&data) {
						Ok(context) => {
							let result = handler.handle(&context);
							Ok(Response::with((status::Ok, serde_json::to_string(&result).unwrap())))
						},
						Err(_) => Ok(Response::with((status::BadRequest, "invalid json")))
					}
				},
				Err(_) => Ok(Response::with((status::BadRequest, "invalid data provided")))
			}
		}).http(&host).unwrap());
		println!("Listening on {}", &host);
	}
}
