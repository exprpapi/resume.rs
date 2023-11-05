use clap::{Parser, Subcommand};
use crate::{
  Error,
  pdf::Session,
};

#[derive(Debug, Parser)]
#[command(about = "resume generator")]
pub struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
  Build { src: Option<String> },
  Watch { src: Option<String> },
}

impl Cli {
  pub fn run() -> Error {
    Cli::parse().command.unwrap_or_default().exec()
  }
}

impl Default for Commands {
  fn default() -> Self {
    Commands::Build { src: None }
  }
}

impl<'a> Commands {
  fn exec(&'a self) -> Error {
    let or_default = |s: &'a Option<String>| -> &'a str {
      s.as_deref().unwrap_or("resume.yaml")
    };
    use Commands::*;
    match self {
      Build { src } => Session::new(or_default(src)).build(),
      Watch { src } => Session::new(or_default(src)).watch(),
    }
  }
}
