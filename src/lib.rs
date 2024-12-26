mod cli;
mod process;

pub use cli::{CsvOpts, GenpassOpts, Opts, SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
