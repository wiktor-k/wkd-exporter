use std::path::PathBuf;

use dir_diff::is_different;
use rstest::rstest;
use tempfile::tempdir;
use testresult::TestResult;
use wkd_exporter::{run, Args};

#[rstest]
fn main(#[files("tests/test-cases/*.pgp")] keyring: PathBuf) -> TestResult {
    let mut expected_dir = keyring.clone();
    expected_dir.set_extension("");
    let keyring = std::fs::File::open(keyring)?;
    let output_dir = tempdir()?.into_path();
    eprintln!(
        "Comparing {} and {}",
        expected_dir.display(),
        output_dir.display()
    );
    run(
        Args {
            well_known: output_dir.clone(),
        },
        keyring,
    )?;
    assert!(
        !is_different(expected_dir, output_dir)?,
        "actual dir has differing content from the expected dir"
    );
    Ok(())
}
