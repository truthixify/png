use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser)]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Parser)]
pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct PrintArgs {
    pub file_path: String,
}
