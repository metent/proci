use axum::extract::State;
use axum::http::Uri;
use axum::routing::get;
use axum::Router;
use serde::de::value::Error as DeError;
use serde::de::value::MapDeserializer;
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Env {
	#[serde(with = "http_serde::uri")]
	container_registry: Uri,
	#[serde(with = "http_serde::uri")]
	auth_endpoint: Uri,
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

	let app = Router::new().with_state(Refs::new()?).route("/", get(root));

	let listener = TcpListener::bind("0.0.0.0:3000").await?;
	axum::serve(listener, app).await?;
	Ok(())
}

async fn root() -> &'static str {
	"Hello, World!"
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("IO Error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Config Error: {0}")]
	ConfigError(#[from] DeError),
}
