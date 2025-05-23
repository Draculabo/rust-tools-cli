use clap::Parser;
use rust_tools_cli::{
    Base64SubCommand, Opts, SubCommand, process_base64_decode, process_base64_encode, process_csv,
    process_genpass,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::Genpass(opts) => {
            process_genpass(
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
                opts.length,
            )?;
        }
        SubCommand::Base64(sub_cmd) => match sub_cmd {
            Base64SubCommand::Encode(opts) => {
                process_base64_encode(opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_base64_decode(opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
