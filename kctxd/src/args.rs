use clap::{arg, Command};
use common::config::Config;
use dirs::config_local_dir;
use thiserror::Error;

use std::path::PathBuf;

pub struct Args{
  c: Command,

  // config_path: String
}

pub fn new(mut cfg: &Config) -> Result<Args, ArgsError> {
  let default_config_dir: PathBuf = match config_local_dir() {
    Some(mut x) => {
      x.push("");
      x
      // let mut x0 = x.clone();
      // x0.push("");
      // x0.to_str().unwrap()
    },
    None => PathBuf::new()
  };

  let default_config_dir: String = default_config_dir.to_string_lossy().to_string();

  let cmd = Command::new(env!("CARGO_CRATE_NAME"))
    .long_version("...")
    .arg(arg!(--config <FILE> "").default_value(default_config_dir));
  let a: Args = Args{
    c: cmd,
    // config_path: default_config_dir,
  };
  Ok(a)
}

impl Args {
  pub fn parse(self) {
    match self.c.get_matches() {
        _ => {},
    }
  }
}

#[derive(Debug, Error)]
pub enum ArgsError {
  #[error("")]
  NoConfigDirectory
}