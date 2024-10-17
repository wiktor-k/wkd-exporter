use std::path::PathBuf;

use clap::Parser;
use wkd_exporter::{cli::Cli, export, Error, Options};

#[derive(Debug, Parser)]
pub struct Args {
    /// Well known directory that will be the output of the exporting process.
    well_known: PathBuf,

    /// Only export the following domain (may be given multiple times).
    #[clap(long, value_parser)]
    domain: Option<Vec<String>>,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let options = Options::default().set_allowed_domains(
        args.domain
            .as_ref()
            .map(|domains| domains.iter().map(|domain| &**domain).collect()),
    );

    export(std::io::stdin(), args.well_known, options)?;
    Ok(())
}
