use std::path::PathBuf;

use dir_diff::is_different;
use rstest::rstest;
use tempfile::tempdir;
use testresult::TestResult;
use wkd_exporter::{export, Options, Variant};

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

#[rstest]
fn archlinux_direct(
    #[files("tests/test-cases-archlinux.org-direct/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        Options::default().set_variant(Variant::Direct("archlinux.org")),
    )
}

#[rstest]
fn append_options(
    #[files("tests/test-cases-default-append/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(keyring, Options::default().set_append(true))
}

#[rstest]
fn archlinux_domain_filter_append(
    #[files("tests/test-cases-archlinux.org-append/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        Options::default()
            .set_allowed_domains(vec!["archlinux.org"])
            .set_append(true),
    )
}

#[rstest]
fn archlinux_direct_append(
    #[files("tests/test-cases-archlinux.org-direct-append/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        Options::default()
            .set_variant(Variant::Direct("archlinux.org"))
            .set_append(true),
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
