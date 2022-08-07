mod pm;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// Get a value
  #[clap(short, long)]
  get: String,
}

fn main() {
  let args = Args::parse();

  let value = pm::get(args.get);

  println!("The value is {}", value);
}
