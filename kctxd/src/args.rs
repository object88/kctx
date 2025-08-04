use std::path::PathBuf;

use clap::{arg, builder::{EnumValueParser, PossibleValue}, Arg, Command, ValueEnum};
use common::config::Config;
use dirs::config_local_dir;
use log::LevelFilter;
use thiserror::Error;

/// ArgLevelFilter is a newtype for LevelFilter, so that ValueEnum can be
/// implemented
#[derive(Clone)]
struct ArgLevelFilter(pub LevelFilter);

// ValueEnum is necessary for EnumValueParser
impl ValueEnum for ArgLevelFilter {
  fn value_variants<'a>() -> &'a [Self] {
    &[
      ArgLevelFilter(LevelFilter::Off),
      ArgLevelFilter(LevelFilter::Error),
      ArgLevelFilter(LevelFilter::Warn),
      ArgLevelFilter(LevelFilter::Info),
      ArgLevelFilter(LevelFilter::Debug),
      ArgLevelFilter(LevelFilter::Trace)
    ]
  }

  fn to_possible_value(&self) -> Option<PossibleValue> {
    Some(PossibleValue::new(Into::<LevelFilter>::into(self.0).to_string().to_lowercase()))
  }
}

pub struct Args<'a>{
  c: Command,

  cfg: &'a mut Config,

  // config_path: String
}

pub fn new(cfg: &mut Config) -> Result<Args, ArgsError> {
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
  let default_log_level: String = cfg.server.log_level.to_string().to_lowercase();

  let cmd = Command::new(env!("CARGO_CRATE_NAME"))
    .long_version("...")
    .arg(arg!(--config <FILE> "").default_value(default_config_dir))
    .arg(Arg::new("loglevel")
      .long("log-level")
      .required(false)
      .default_value(default_log_level)
      .help("Set the loglevel")
      .long_help("Set the loglevel. 'trace' is the most verbose and 'off' the least verbose")
      .value_parser(EnumValueParser::<ArgLevelFilter>::new())
    );

  let a: Args = Args{
    c: cmd,
    cfg,
    // config_path: default_config_dir,
  };
  Ok(a)
}

impl Args<'_> {
  pub fn parse(self) {
    match self.c.get_matches() {
      a => {
        let log_level_value: &LevelFilter = &a.get_one::<ArgLevelFilter>("loglevel").unwrap().0;
        
        self.cfg.server.log_level = *log_level_value;
      },
    }
  }
}

#[derive(Debug, Error)]
pub enum ArgsError {
  #[error("")]
  NoConfigDirectory
}