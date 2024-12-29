use super::Refs;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn tags(
	Path((user, image)): Path<(String, String)>,
	State(refs): State<Refs>,
) -> Result<String, Error> {
	let query = TokenQuery {
		service: refs
			.env
			.container_registry
			.host_str()
			.ok_or(Error::InvalidRegistry)?,
		scope: &format!("repository:{}/{}:pull", user, image),
	};
	let response: TokenResponse = Client::new()
		.get(refs.env.auth_endpoint)
		.query(&query)
		.send()
		.await?
		.error_for_status()?
		.json()
		.await?;
	Ok(response.token)
}

#[derive(Serialize)]
struct TokenQuery<'a> {
	service: &'a str,
	scope: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
	token: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Registry connection error: {0}")]
	RegistryConnection(#[from] reqwest::Error),
	#[error("Invalid registry URL")]
	InvalidRegistry,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("{}", self);
		(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
	}
}
