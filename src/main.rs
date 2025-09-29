use std::process::{ExitCode, Termination};

use clap::Parser;
use wkd_exporter::{Options, Variant, cli::Cli, export};

fn main() -> ExitCode {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();

    let options = Options::default()
        .set_allowed_domains(
            args.domain
                .as_ref()
                .map(|domains| domains.iter().map(|domain| &**domain).collect()),
        )
        .set_variant(
            args.direct
                .as_ref()
                .map(|domain| Variant::Direct(domain))
                .unwrap_or_default(),
        )
        .set_append(args.append);

    if let Err(e) = export(std::io::stdin(), args.well_known, &options) {
        tracing_log_error::log_error!(e);
        e.report()
    } else {
        ExitCode::SUCCESS
    }
}
