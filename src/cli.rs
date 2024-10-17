//! Command line features.

use std::path::PathBuf;

use clap::Parser;

/// WKD exporter.
///
/// Exports a given OpenPGP keyring into a directory structure
/// required by the Web Key Directory specification.
#[derive(Debug, Parser)]
#[command(name = "wkd-exporter")]
pub struct Cli {
    /// Well known directory that will be the output of the exporting process.
    pub well_known: PathBuf,

    /// Only export the following domain (may be given multiple times).
    ///
    /// By default all domains are exported but if this option is
    /// given only the selected ones will be exported.
    #[clap(long, value_parser)]
    pub domain: Option<Vec<String>>,
}
