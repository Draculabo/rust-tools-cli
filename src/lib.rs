mod cli;
mod process;

pub use cli::{Base64SubCommand, CsvOpts, GenpassOpts, Opts, SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
pub use process::{process_base64_decode, process_base64_encode};
