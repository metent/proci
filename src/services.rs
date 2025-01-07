use super::Refs;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use rinja::Template;
use std::sync::Arc;

pub async fn tags(
	Path((user, image)): Path<(String, String)>,
	State(refs): State<Arc<Refs>>,
) -> Result<Html<String>, Error> {
	let image_path = user + "/" + &image;
	let tags_response = refs.client.tags(&image_path).await?;
	Ok(Html(
		TagsTemplate {
			tags: tags_response.tags(),
			image_path: &image_path,
			blob_suffix: &refs.blob_suffix,
		}
		.render()?,
	))
}

#[derive(Template)]
#[template(path = "tags.html")]
struct TagsTemplate<'a> {
	tags: &'a [String],
	image_path: &'a str,
	blob_suffix: &'a str,
}

pub async fn blob(
	Path((user, image, tag)): Path<(String, String, String)>,
	State(refs): State<Arc<Refs>>,
) -> Result<Redirect, Error> {
	let image_path = user + "/" + &image;
	Ok(Redirect::to(
		&refs.client.blob_url(&image_path, &tag).await?,
	))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Registry error: {0}")]
	RegistryConnection(#[from] crate::client::Error),
	#[error("Templating error: {0}")]
	Templating(#[from] rinja::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("{}", self);
		(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
	}
}
