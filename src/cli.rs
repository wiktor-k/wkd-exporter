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
    #[clap(long)]
    pub domain: Option<Vec<String>>,

    /// Append incoming certificates to the ones already existing in
    /// `.well-known`.
    ///
    /// Enable this option if you have multiple certificates
    /// containing the same local-part and want them all to be part of
    /// the WKD export.
    ///
    /// Note that running the same process twice with append and the
    /// same `.well-known` will duplicate the certificates there.
    #[clap(long)]
    pub append: bool,

    /// Select direct WKD variant with a filter for this single domain.
    #[clap(long)]
    pub direct: Option<String>,
}
