use crate::Credentials;
use either::IntoEither;
use oci_spec::distribution::TagList;
use oci_spec::image::{ImageManifest, MediaType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

pub struct OciClient {
	client: Client,
	registry: String,
	registry_url: Url,
	auth_url: Url,
	credentials: Option<Credentials>,
	media_type: MediaType,
}

impl OciClient {
	pub fn new(
		registry: String,
		auth_url: Url,
		credentials: Option<Credentials>,
		media_type: MediaType,
	) -> Result<Self, url::ParseError> {
		let registry_url = Url::parse(&format!("https://{registry}"))?;
		Ok(OciClient {
			client: Client::new(),
			registry,
			registry_url,
			auth_url,
			credentials,
			media_type,
		})
	}

	pub async fn tags(&self, image_path: &str) -> Result<TagList, Error> {
		Ok(self
			.client
			.get(format!("{}/v2/{}/tags/list", self.registry_url, image_path))
			.bearer_auth(&self.auth(image_path).await?)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?)
	}

	pub async fn blob_url(&self, image_path: &str, tag: &str) -> Result<String, Error> {
		let token = self.auth(image_path).await?;
		let manifest: ImageManifest = self
			.client
			.get(format!(
				"{}/v2/{}/manifests/{}",
				self.registry_url, image_path, tag
			))
			.bearer_auth(&token)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?;

		let digest = manifest
			.layers()
			.iter()
			.find(|l| l.media_type() == &self.media_type)
			.ok_or(Error::MediaNotFound)?
			.digest();

		Ok(self
			.client
			.get(format!(
				"{}/v2/{}/blobs/{}",
				self.registry_url, image_path, digest
			))
			.bearer_auth(&token)
			.send()
			.await?
			.url()
			.to_string())
	}

	async fn auth(&self, image_path: &str) -> Result<String, Error> {
		let query = TokenQuery {
			service: &self.registry,
			scope: &format!("repository:{}:pull", image_path),
		};
		let (username, password) = match &self.credentials {
			Some(Credentials { username, password }) => (username.as_str(), Some(password)),
			None => ("", None),
		};

		let response: TokenResponse = self
			.client
			.get(self.auth_url.clone())
			.query(&query)
			.into_either(self.credentials.is_some())
			.right_or_else(|req| req.basic_auth(username, password))
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?;

		Ok(response.token)
	}
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
	#[error("Cannot establish connection to registry: {0}")]
	Connection(#[from] reqwest::Error),
	#[error("Media not found")]
	MediaNotFound,
}
