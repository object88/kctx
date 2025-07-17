use clap::{arg, builder::OsStr, Command};
use dirs::config_local_dir;
use thiserror::Error;

use std::path::PathBuf;

pub struct Args{
  c: Command,

  config_path: OsStr
}

pub fn new() -> Result<Args, ArgsError> {
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

  let default_config_dir: OsStr = default_config_dir.as_os_str().into();

  let cmd = Command::new(env!("CARGO_CRATE_NAME"))
    .long_version("...")
    .arg(arg!(--config <FILE> "").default_missing_value(&default_config_dir));
  let a: Args = Args{
    c: cmd,
    config_path: default_config_dir,
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