use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    match args::Cli::parse().command {
        args::Commands::Encode(args) => commands::encode(args),
        args::Commands::Decode(args) => commands::decode(args),
        args::Commands::Remove(args) => commands::remove(args),
        args::Commands::Print(args) => commands::print_chunks(args),
    }
}
