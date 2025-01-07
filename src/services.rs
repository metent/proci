use super::Refs;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

pub async fn tags(
	Path((user, image)): Path<(String, String)>,
	State(refs): State<Refs>,
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Registry connection error: {0}")]
	RegistryConnection(#[from] reqwest::Error),
	#[error("Templating error: {0}")]
	Templating(#[from] rinja::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("{}", self);
		(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
	}
}
