#![cfg_attr(coverage, feature(coverage_attribute))]
#![cfg_attr(coverage, coverage(off))]

use std::{env::var, path::PathBuf};

use cargo_toml::Manifest;
use clap::{Parser, ValueEnum};
use clap_allgen::{render_manpages, render_shell_completions};
use enum_values::EnumValues as _;
use roff::{Inline, Roff, bold};
use wkd_exporter::cli::Cli as WkdExporterCli;

#[derive(Parser, Debug)]
enum Commands {
    /// Generate auxiliary artifacts.
    Generate(GenerateCommand),
}

#[derive(Parser, Debug)]
struct GenerateCommand {
    /// The type of the artifact to generate.
    kind: ArtifactKind,

    /// Optional output directory root.
    ///
    /// This command will generate a subdirectory based on the artifact kind.
    ///
    /// Defaults to the `target` subdirectory of the current directory (`$PWD`) unless `$CARGO_TARGET_DIR` is set.
    #[clap(env = "CARGO_TARGET_DIR")]
    output_root: Option<PathBuf>,
}

#[derive(Parser, Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "snake_case")]
enum ArtifactKind {
    /// Man pages.
    Manpages,

    /// Shell completions for all available shells.
    ShellCompletions,
}

fn main() -> testresult::TestResult {
    let args = Commands::parse();
    match args {
        Commands::Generate(cmd) => {
            let output = cmd
                .output_root
                .unwrap_or_else(|| {
                    PathBuf::from(var("PWD").unwrap_or(".".to_owned())).join("target")
                })
                .join(cmd.kind.to_possible_value().unwrap().get_name());

            match cmd.kind {
                ArtifactKind::Manpages => {
                    render_manpages::<WkdExporterCli>(&output)?;

                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .open(output.join("wkd-exporter.1"))?;

                    let mut roff = Roff::default();
                    roff.control("SH", ["EXIT STATUS"]);
                    roff.text(["On success the application returns 0.".into()]);
                    roff.text([Inline::LineBreak]);
                    roff.text(["On failure the following exit codes are used:".into()]);

                    for variant in wkd_exporter::Error::variants() {
                        roff.text([Inline::LineBreak]);
                        roff.text([bold(variant.value.to_string())]);
                        roff.control("RS", []);
                        roff.text([variant.doc.into()]);
                        roff.control("RE", []);
                    }

                    let readme = std::fs::read_to_string("README.md")?;
                    let readme = readme
                        .split("\n")
                        .skip_while(|s| !s.contains("<!-- BEGIN CLI EXAMPLE -->"))
                        .skip(1)
                        .take_while(|s| !s.contains("<!-- END CLI EXAMPLE -->"));

                    roff.control("SH", ["EXAMPLES"]);
                    roff.text(
                        readme
                            .into_iter()
                            .flat_map(|s| [s.into(), Inline::LineBreak])
                            .collect::<Vec<_>>(),
                    );

                    let manifest = Manifest::from_path("./Cargo.toml")?;

                    roff.control("SH", ["BUGS"]);
                    roff.text([
                        "Please report all bugs to ".into(),
                        manifest
                            .package()
                            .repository()
                            .expect("repository to be set")
                            .into(),
                    ]);

                    roff.control("SH", ["AUTHORS"]);
                    roff.text(
                        manifest
                            .package()
                            .authors()
                            .iter()
                            .flat_map(|s| [s.into(), Inline::LineBreak])
                            .collect::<Vec<_>>(),
                    );

                    roff.control("SH", ["COPYRIGHT"]);
                    roff.text([manifest
                        .package()
                        .license()
                        .expect("published software to have a license")
                        .into()]);

                    roff.to_writer(&mut file)?;
                }
                ArtifactKind::ShellCompletions => {
                    render_shell_completions::<WkdExporterCli>(&output)?;
                }
            }
        }
    }
    Ok(())
}
