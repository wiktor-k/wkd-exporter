use clap::Parser;

use wkd_exporter::{run, Args};

fn main() -> testresult::TestResult {
    let args = Args::parse();
    run(args, std::io::stdin())?;
    Ok(())
}
