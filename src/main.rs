mod cli;
mod pm;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
  let instance = Cli::parse();

  match instance.command {
    Some(Commands::Init {}) => {
      pm::init();
    }
    Some(Commands::Set { key, value }) => {
      pm::set(key, value);
    }
    Some(Commands::Get { key }) => {
      pm::get(key);
    }
    None => {}
  }
}
