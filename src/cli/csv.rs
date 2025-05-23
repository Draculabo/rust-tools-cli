use std::fmt::Display;
use std::str::FromStr;

use clap::Parser;

use crate::cli::verify_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, value_parser = parse_output_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl From<&str> for OutputFormat {
    fn from(format: &str) -> Self {
        match format.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "yaml" => OutputFormat::Yaml,
            _ => panic!("Invalid output format: {}", format),
        }
    }
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        Ok(OutputFormat::from(format))
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
