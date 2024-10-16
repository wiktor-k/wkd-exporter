use std::path::PathBuf;

use clap::Parser;
use wkd_exporter::{export, Error};

#[derive(Debug, Parser)]
pub struct Args {
    well_known: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    export(args.well_known, std::io::stdin())?;
    Ok(())
}
