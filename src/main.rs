use clap::Parser;
use rust_tools_cli::{Opts, SubCommand, process_csv, process_genpass};

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
    }
    Ok(())
}
