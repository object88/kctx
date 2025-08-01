use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	pub server: Server,
}

impl Config {
	pub fn new() -> Self {
		Config { server: Server::new(), }
	}
}

#[derive(Deserialize)]
pub struct Server {
	pub api_http: Http,
	pub health_http: Http,
}

impl Server {
	pub fn new() -> Self {
		Server {
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

#[derive(Deserialize)]
pub struct Http {
	pub host: String,
	pub port: u16,
}
