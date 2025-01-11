# pr**oci**

pro(x)ci is a proxy with a simple web-interface for facilitating access to OCI blobs which are normally accessible only through CLI tools (like oras) or by directly calling the OCI distribution APIs.

The downloaded file will always be named after its sha256 digest, as most container registries don't implement CORS or do not allow `Content-Disposition` to be set. A simple, yet efficient workaround is setting up a CDN with the proci server as the origin. If your CDN supports this, you can use the `Response-Content-Disposition` header to set the `Content-Disposition` response header sent by the registry backend.

Check the [sample .env file](.env.example) for configuration. proci (currently) doesn't supported parsing .env files. You may use .cargo/config.toml for setting these environment variables or if you're using docker/docker compose, you can use the `env-file` setting for reading the .env file.
