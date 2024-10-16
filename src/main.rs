use std::path::PathBuf;

use clap::Parser;
use wkd_exporter::{export, Error, Options};

#[derive(Debug, Parser)]
pub struct Args {
    well_known: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    export(std::io::stdin(), args.well_known, Options::default())?;
    Ok(())
}
