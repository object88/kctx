use std::{net::{IpAddr, SocketAddr}, str::FromStr};

use async_trait::async_trait;
use axum::{routing::get, Router};
use common::config::Http as HttpConfig;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::lifecycle::Runnable;

#[derive(Debug, Error)]
pub enum Error {
  #[error("")]
  BadAddr,
}

pub struct Builder {
  cfg: HttpConfig
}

pub struct Http {
  app: Option<Router>,
  listener: Option<TcpListener>,
}

pub fn new(cfg: HttpConfig) -> Builder {
  return Builder{
    cfg
  }
}

impl Builder {
  pub async fn build(self) -> Result<Http, Error> {
    let ip_addr = match IpAddr::from_str(&self.cfg.host) {
      Ok(x) => x,
      Err(_e) => {
        return Err(Error::BadAddr)
      }
    };
    let addr = SocketAddr::new(ip_addr, self.cfg.port);

    let app = Router::new().route("/", get(|| async { "Hello, World" }));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    return Ok(Http{
      app: Some(app),
      listener: Some(listener),
    })
  }
}

#[async_trait]
impl Runnable for Http {
  async fn run(&mut self, cancel_token: CancellationToken) {
    println!("http started");
    let app = self.app.take();
    let listener = self.listener.take();
    axum::serve(listener.unwrap(), app.unwrap()).with_graceful_shutdown(async move {
      cancel_token.cancelled().await 
    }).await.unwrap();
    println!("http exited");
  }
}

