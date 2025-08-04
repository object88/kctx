use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
	pub server: Server,
}

impl Config {
	pub fn new() -> Self {
		Config { server: Server::new(), }
	}
}

#[derive(Debug, Deserialize)]
pub struct Server {
	pub log_level: LevelFilter,
	pub api_http: Http,
	pub health_http: Http,
}

impl Server {
	pub fn new() -> Self {
		Server {
			log_level: LevelFilter::Warn,
			api_http: Http{
				host: "0.0.0.0".to_string(),
				port: 3000,
			},
			health_http: Http{
				host: "0.0.0.0".to_string(),
				port: 3001,
			},
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct Http {
	pub host: String,
	pub port: u16,
}
