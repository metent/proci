use either::IntoEither;
use oci_spec::distribution::TagList;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone)]
pub struct OciClient {
	client: Client,
	registry: String,
	registry_url: Url,
	auth_url: Url,
	credentials: Option<(String, String)>,
}

impl OciClient {
	pub fn new(
		mut registry: String,
		auth_url: Url,
		credentials: Option<(String, String)>,
	) -> Result<Self, url::ParseError> {
		registry.insert_str(0, "https://");
		let registry_url = Url::parse(&registry)?;
		Ok(OciClient {
			client: Client::new(),
			registry,
			registry_url,
			auth_url,
			credentials,
		})
	}

	pub async fn tags(&self, image_path: &str) -> Result<TagList, Error> {
		self.client
			.get(format!("{}/v2/{}/tags/list", self.registry_url, image_path))
			.bearer_auth(&self.auth(image_path).await?)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await
	}

	async fn auth(&self, image_path: &str) -> Result<String, Error> {
		let query = TokenQuery {
			service: &self.registry,
			scope: &format!("repository:{}:pull", image_path),
		};
		let (username, password) = match &self.credentials {
			Some((u, p)) => (u.as_str(), Some(p)),
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
