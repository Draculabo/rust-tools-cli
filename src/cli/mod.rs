mod csv;
mod genpass;
use clap::Parser;
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenpassOpts;

#[derive(Debug, Parser)]
#[command(name = "rust-tools-cli", author = "draculabo", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "csv to json")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    Genpass(GenpassOpts),
}
