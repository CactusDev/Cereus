//! Responses from the API

use packet::Context;

pub struct UserAttributes {
	
}

pub struct UserResponse {
	id: String
}

pub struct CommandAttributes {
	createdAt: String,
	name:      String,
	response:  Context,
	count:     usize,
	enabled:   bool,
	token:     String
}

pub struct CommandResponse {
	id:         String,
	attributes: CommandAttributes,
	type:        String
}
