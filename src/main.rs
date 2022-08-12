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
  /// Init the local pm instance
  Init {},

  /// Set a value on the password manager
  Set {
    /// Key of the value to store
    #[clap(value_parser)]
    key: String,

    /// Value to store
    #[clap(value_parser)]
    value: String,
  },

  /// Get a value
  Get {
    /// Key of the value to obtain
    #[clap(value_parser)]
    key: String,
  }
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
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
