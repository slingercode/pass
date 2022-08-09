mod pm;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// Set a value on the password manager
  Set {
    /// Key of the value to store
    #[clap(value_parser)]
    key: String,

    /// Value to store
    #[clap(value_parser)]
    value: String,
  }
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Some(Commands::Set { key, value }) => {
      pm::set(key, value);
    }
    None => {}
  }
}
