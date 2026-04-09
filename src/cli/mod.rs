use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub first: String,
    pub second: String,
}

pub fn return_args() {
    let args = Cli::parse();
    println!("first: {:?}, second: {:?}", args.first, args.second);
}