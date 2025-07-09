use std::path::PathBuf;

use dir_diff::is_different;
use rstest::rstest;
use tempfile::tempdir;
use testresult::TestResult;
use wkd_exporter::{Options, Variant, export};

#[rstest]
#[test_log::test]
fn default_options(#[files("tests/test-cases-default/*.pgp")] keyring: PathBuf) -> TestResult {
    test_export_with_options(keyring, &Options::default())
}

#[rstest]
#[test_log::test]
fn archlinux_domain_filter(
    #[files("tests/test-cases-archlinux.org/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        &Options::default().set_allowed_domains(vec!["archlinux.org"]),
    )
}

#[rstest]
#[test_log::test]
fn archlinux_direct(
    #[files("tests/test-cases-archlinux.org-direct/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        &Options::default().set_variant(Variant::Direct("archlinux.org")),
    )
}

#[rstest]
#[test_log::test]
fn append_options(
    #[files("tests/test-cases-default-append/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(keyring, &Options::default().set_append(true))
}

#[rstest]
#[test_log::test]
fn archlinux_domain_filter_append(
    #[files("tests/test-cases-archlinux.org-append/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        &Options::default()
            .set_allowed_domains(vec!["archlinux.org"])
            .set_append(true),
    )
}

#[rstest]
#[test_log::test]
fn archlinux_direct_append(
    #[files("tests/test-cases-archlinux.org-direct-append/*.pgp")] keyring: PathBuf,
) -> TestResult {
    test_export_with_options(
        keyring,
        &Options::default()
            .set_variant(Variant::Direct("archlinux.org"))
            .set_append(true),
    )
}

fn test_export_with_options(keyring: PathBuf, options: &Options) -> TestResult {
    let mut expected_dir = keyring.clone();
    expected_dir.set_extension("");
    let keyring = std::fs::File::open(keyring)?;
    let output_dir = tempdir()?.keep();
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
