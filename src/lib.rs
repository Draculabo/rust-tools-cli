mod opts;
mod process;

pub use opts::SubCommand;
pub use opts::{CsvOpts, GenpassOpts, Opts};
pub use process::process_csv;
pub use process::process_genpass;
