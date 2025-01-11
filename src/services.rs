use super::Refs;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use rinja::Template;
use std::sync::Arc;

pub async fn tags(
	Path((user, image)): Path<(String, String)>,
	State(refs): State<Arc<Refs>>,
) -> Result<impl IntoResponse, Error> {
	let image_path = user + "/" + &image;
	let tags_response = refs.client.tags(&image_path).await?;
	Ok((
		[(header::CACHE_CONTROL, "s-maxage=60, max-age=0")],
		Html(
			TagsTemplate {
				tags: tags_response.tags(),
				image_path: &image_path,
				blob_suffix: &refs.blob_suffix,
			}
			.render()?,
		),
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
	Path((user, image, suffixed_tag)): Path<(String, String, String)>,
	State(refs): State<Arc<Refs>>,
) -> Result<impl IntoResponse, Error> {
	let image_path = user + "/" + &image;
	let tag = suffixed_tag
		.strip_suffix(&refs.blob_suffix)
		.ok_or(Error::InvalidRoute)?;
	let blob_url = refs.client.blob_url(&image_path, &tag).await?;

	let disposition = format!(r#"attachment; filename="{suffixed_tag}""#);
	let headers = [("response-content-disposition", disposition)];

	Ok((headers, Redirect::to(&blob_url)))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Registry error: {0}")]
	RegistryConnection(#[from] crate::client::Error),
	#[error("Templating error: {0}")]
	Templating(#[from] rinja::Error),
	#[error("Invalid Route")]
	InvalidRoute,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("{}", self);
		match self {
			Error::InvalidRoute => (StatusCode::NOT_FOUND, "Not Found"),
			_ => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"),
		}
		.into_response()
	}
}
