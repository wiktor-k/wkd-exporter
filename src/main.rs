use clap::Parser;
use wkd_exporter::{cli::Cli, export, Error, Options, Variant};

fn main() -> Result<(), Error> {
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

    export(std::io::stdin(), args.well_known, options)?;
    Ok(())
}
