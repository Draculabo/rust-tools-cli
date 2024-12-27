use std::str::FromStr;

use clap::{Parser, command};

use crate::cli::verify_file;

#[derive(Debug, Parser)]
pub struct BaseEncode64Opts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64ODecodepts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 encode")]
    Encode(BaseEncode64Opts),
    #[command(name = "decode", about = "base64 decode")]
    Decode(Base64ODecodepts),
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format: {}", format)),
        }
    }
}
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}
