#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use std::{fs::OpenOptions, io::Read, path::Path, str::FromStr};

use email_address::EmailAddress;
use pgp::{ser::Serialize, Deserializable, SignedPublicKey};

/// Error when exporting the keyring.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// PGP processing failed.
    #[error("PGP processing error occurred: {0}")]
    Pgp(#[from] pgp::errors::Error),

    /// I/O operation failed.
    #[error("I/O error occurred: {0}")]
    Io(#[from] std::io::Error),
}

/// Exporting options.
#[derive(Debug, Default)]
pub struct Options {}

/// Exports a keyring file (`input`) to a given well known directory.
///
/// # Examples
///
/// ```
/// # fn main() -> testresult::TestResult {
/// use wkd_exporter::{export, Options};
///
/// export(
///     std::fs::File::open("tests/test-cases/simple.pgp")?,
///     "/tmp/well-known",
///     Options::default(),
/// )?;
/// # Ok(()) }
/// ```
pub fn export(
    keyring: impl Read,
    well_known: impl AsRef<Path>,
    _options: Options,
) -> Result<(), Error> {
    let openpgpkey = well_known.as_ref().join("openpgpkey");
    std::fs::create_dir_all(&openpgpkey)?;
    let iterator = SignedPublicKey::from_reader_many(keyring)?.0;
    for key in iterator {
        let key = key?;
        for (encoded_local, domain) in key
            .details
            .users
            .iter()
            .flat_map(|user| EmailAddress::from_str(&user.id.id().to_string()))
            .map(|email| {
                use sha1::Digest;
                let local_part = email.local_part().to_lowercase();
                let mut digest = sha1::Sha1::new();
                digest.update(local_part.as_bytes());
                (
                    zbase32::encode(&digest.finalize()[..]),
                    email.domain().to_string(),
                )
            })
        {
            let domain = openpgpkey.join(&domain);
            let hu = domain.join("hu");
            std::fs::create_dir_all(&hu)?;

            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(domain.join("policy"))?;

            let mut key_file = std::fs::File::create(hu.join(encoded_local))?;
            key.to_writer(&mut key_file)?;
        }
    }

    Ok(())
}
