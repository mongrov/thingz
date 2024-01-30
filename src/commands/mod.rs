extern crate log;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// config file
    #[arg(short, long, value_name="config file", default_value="")]
    pub config: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// mqtt client service
    Mqtt,
    /// S3 client service
    S3,
}
