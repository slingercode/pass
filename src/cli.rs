use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
    },
}
