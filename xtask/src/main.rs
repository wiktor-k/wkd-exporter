#![cfg_attr(coverage, feature(coverage_attribute))]
#![cfg_attr(coverage, coverage(off))]

use std::{env::var, path::PathBuf};

use clap::{Parser, ValueEnum};
use clap_allgen::{render_manpages, render_shell_completions};
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

fn main() -> Result<(), clap_allgen::Error> {
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
                ArtifactKind::Manpages => render_manpages::<WkdExporterCli>(&output),
                ArtifactKind::ShellCompletions => {
                    render_shell_completions::<WkdExporterCli>(&output)
                }
            }
        }
    }
}
