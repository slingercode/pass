mod cli;
mod dir;
mod pm;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli_instance = Cli::parse();

    match cli_instance.command {
        Some(Commands::Init {}) => {
            pm::init();
        }
        Some(Commands::Set { key, value }) => {
            pm::set(key, value);
        }
        Some(Commands::Get { key }) => {
            println!("{}", pm::get(key));
        }
        None => {}
    }
}
