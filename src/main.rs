#[macro_use] extern crate nickel;
extern crate hyper;

use nickel::status::StatusCode;
use nickel::{Nickel, Mountable, StaticFilesHandler, QueryString, MediaType};
use hyper::header::{AccessControlAllowOrigin};

use serde_json;
use crate::get_metadata::get_metadata;

mod get_metadata;

fn main() {
	let mut server = Nickel::new();

	server.utilize(router! {
		get "/api" => |req, mut res| {
			let url = req.query().get("url").unwrap();
			res.set(MediaType::Json);
			res.set(AccessControlAllowOrigin::Any);
			serde_json::to_value(get_metadata(url)).map_err(|e| (StatusCode::InternalServerError, e))
		}
	});

	server.mount("/", StaticFilesHandler::new("public"));

	// server.utilize(StaticFilesHandler::new("public"));

	server.listen("127.0.0.1:6767").unwrap();
}