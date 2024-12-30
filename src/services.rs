use super::Refs;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::ops::Add;

pub async fn tags(
	Path((user, image)): Path<(String, String)>,
	State(refs): State<Refs>,
) -> Result<String, Error> {
	let tags = refs.client.tags(&user.add("/").add(&image)).await?;
	Ok(tags.tags().concat())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Registry connection error: {0}")]
	RegistryConnection(#[from] reqwest::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("{}", self);
		(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
	}
}
