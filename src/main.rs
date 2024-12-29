mod services;

use crate::services::tags;
use axum::routing::get;
use axum::Router;
use serde::de::value::Error as DeError;
use serde::de::value::MapDeserializer;
use serde::Deserialize;
use tokio::net::TcpListener;
use url::Url;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Env {
	container_registry: Url,
	auth_endpoint: Url,
	#[serde(default, flatten)]
	credentials: Option<String>,
	blob_suffix: String,
}

#[derive(Clone)]
struct Refs {
	env: Env,
}

impl Refs {
	fn new() -> Result<Self, Error> {
		let env = Env::deserialize(MapDeserializer::<_, DeError>::new(std::env::vars()))?;
		return Ok(Refs { env });
	}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	tracing_subscriber::fmt::init();

	let app = Router::new()
		.route("/{user}/{image}", get(tags))
		.with_state(Refs::new()?);

	let listener = TcpListener::bind("0.0.0.0:3000").await?;
	axum::serve(listener, app).await?;
	Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("IO Error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Config Error: {0}")]
	ConfigError(#[from] DeError),
}
