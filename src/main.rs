mod client;
mod services;

use crate::client::OciClient;
use crate::services::{blob, tags};
use axum::routing::get;
use axum::Router;
use oci_spec::image::MediaType;
use serde::de::value::Error as DeError;
use serde::de::value::MapDeserializer;
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Env {
	container_registry: String,
	auth_endpoint: Url,
	#[serde(default, flatten)]
	credentials: Option<(String, String)>,
	blob_suffix: String,
	media_type: MediaType,
}

struct Refs {
	client: OciClient,
	blob_suffix: String,
}

impl Refs {
	fn new() -> Result<Self, Error> {
		let env = Env::deserialize(MapDeserializer::<_, DeError>::new(std::env::vars()))?;
		let client = OciClient::new(
			env.container_registry,
			env.auth_endpoint,
			env.credentials,
			env.media_type,
		)?;
		return Ok(Refs {
			client,
			blob_suffix: env.blob_suffix,
		});
	}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	tracing_subscriber::fmt::init();

	let app = Router::new()
		.route("/{user}/{image}/{tag}", get(blob))
		.route("/{user}/{image}", get(tags))
		.with_state(Arc::new(Refs::new()?));

	let listener = TcpListener::bind("0.0.0.0:3000").await?;
	axum::serve(listener, app).await?;
	Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("IO Error: {0}")]
	Io(#[from] std::io::Error),
	#[error("Config Error: {0}")]
	Config(#[from] DeError),
	#[error("Invalid Registry URL")]
	InvalidRegistry(#[from] url::ParseError),
}
