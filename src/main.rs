use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	tracing_subscriber::fmt::init();

	let app = Router::new().route("/", get(root));

	let listener = TcpListener::bind("0.0.0.0:3000").await?;
	axum::serve(listener, app).await?;
	Ok(())
}

async fn root() -> &'static str {
	"Hello, World!"
}
