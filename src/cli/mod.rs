mod csv;
mod genpass;
use clap::Parser;
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenpassOpts;
use std::path::Path;

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

pub fn verify_file(file_name: &str) -> Result<String, String> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err(format!("File not found: {}", file_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        // assert_eq!(verify_file("test.txt"), Ok("test.txt".to_string()));
        assert_eq!(
            verify_file("not_found.txt"),
            Err("File not found: not_found.txt".to_string())
        );
    }
}
