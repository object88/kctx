use common::config::Config;
use kctxd::{args::{self, Args}, http, lifecycle::{self, Runnable}};
use log::info;
use structured_logger::{async_json::new_writer, Builder};
use thiserror::Error;
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Error)]
pub enum AppError {
	#[error("invalid configuration file")]
	ConfigReadError
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
	let mut c = Config::new();
	let a: Args = args::new(&c).unwrap();
	a.parse();

	// Set up logging
	Builder::with_level("info")
			.with_target_writer("*", new_writer(tokio::io::stdout()))
			.init();

	let token = CancellationToken::new();
	let lifecycle_token = token.clone();

	let builder = http::new(c.server.api_http);
	let http = match builder.build().await {
		Ok(x) => x,
		Err(e) => {
			return Err(AppError::ConfigReadError);
		}
	};

	let status_builder = http::new(c.server.health_http);
	let status_http = match status_builder.build().await {
		Ok(x) => x,
		Err(e) => {
			return Err(AppError::ConfigReadError);
		}
	};

	let v: Vec<Box<dyn Runnable>> = vec![Box::new(http), Box::new(status_http)];

	// Start lifecycle
	let task_handle = tokio::spawn(async move {
		info!("lifecycle started");
		lifecycle::run(lifecycle_token, v).await;
		info!("lifecycle ended");
	});
	
	tokio::select! {
		_ = signal::ctrl_c() => {
			token.cancel();
		},
	}

	// Send shutdown to the lifecycle

	task_handle.await.unwrap();

	Ok(())
}
