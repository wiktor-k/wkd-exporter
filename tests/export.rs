use std::path::PathBuf;

use dir_diff::is_different;
use rstest::rstest;
use tempfile::tempdir;
use testresult::TestResult;
use wkd_exporter::{export, Options};

#[rstest]
fn default_options(#[files("tests/test-cases-default/*.pgp")] keyring: PathBuf) -> TestResult {
    test_export_with_options(keyring, Options::default())
}

#[rstest]
fn archlinux_domain_filter(
    #[files("tests/test-cases-archlinux.org/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        Options::default().set_allowed_domains(vec!["archlinux.org"]),
    )
}

fn test_export_with_options(keyring: PathBuf, options: Options) -> TestResult {
    let mut expected_dir = keyring.clone();
    expected_dir.set_extension("");
    let keyring = std::fs::File::open(keyring)?;
    let output_dir = tempdir()?.into_path();
    eprintln!(
        "Comparing {} and {}",
        expected_dir.display(),
        output_dir.display()
    );
    export(keyring, &output_dir, options)?;
    assert!(
        !is_different(expected_dir, output_dir)?,
        "actual dir has differing content from the expected dir"
    );
    Ok(())
}
