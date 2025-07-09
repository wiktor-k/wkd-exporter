#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::{fs::OpenOptions, io::Read, path::Path, str::FromStr};

use email_address::EmailAddress;
use pgp::{
    composed::{Deserializable, SignedPublicKey},
    ser::Serialize,
    types::KeyDetails as _,
};
use tracing::{debug, info, trace};

#[cfg(feature = "cli")]
#[doc(hidden)]
pub mod cli;

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

/// WKD variant.
///
/// There are two WKD directory structures: direct and advanced.
/// Direct supports only one domain while advanced can support any number of domains.
#[derive(Debug, Default)]
pub enum Variant<'a> {
    /// Advanced variant supporting multiple domains.
    #[default]
    Advanced,

    /// Direct variant supporting just one domain.
    Direct(&'a str),
}

/// Exporting options.
///
/// This struct can be used to adjust the exporting process.
///
/// # Examples
///
/// The following code makes the exporting process filter domains to
/// only these explicitly mentioned. Additionally it supports multiple
/// certificates for the same e-mail address:
///
/// ```
/// use wkd_exporter::{Options, Variant};
///
/// let only_arch = Options::default()
///     .set_allowed_domains(vec!["archlinux.org"])
///     .set_variant(Variant::Advanced)
///     .set_append(true);
/// ```
#[derive(Debug, Default)]
pub struct Options<'a, 'b> {
    allowed_domains: Option<Vec<&'a str>>,

    append: bool,

    variant: Variant<'b>,
}

impl<'a, 'b> Options<'a, 'b>
where
    'b: 'a,
{
    /// Sets a list of allowed domains for the export.
    ///
    /// Setting this option to `None` (the default) exports all domains.
    ///
    /// # Examples
    ///
    /// The following code makes the exporting process filter domains to only
    /// these explicitly mentioned:
    ///
    /// ```
    /// use wkd_exporter::Options;
    ///
    /// let only_arch = Options::default().set_allowed_domains(vec!["archlinux.org"]);
    /// ```
    #[must_use]
    pub fn set_allowed_domains(mut self, allowed_domains: impl Into<Option<Vec<&'a str>>>) -> Self {
        self.allowed_domains = allowed_domains.into();
        self
    }

    /// Check if a given domain is allowed for export.
    #[must_use]
    pub fn is_domain_allowed(&self, domain: &str) -> bool {
        self.allowed_domains
            .as_ref()
            .is_none_or(|domains| domains.contains(&domain))
    }

    /// Set WKD directory variant.
    ///
    /// Setting a direct variant implies that the filter for that one
    /// domain will be applied as well. There is no need to use
    /// [`Self::set_allowed_domains`] when using [`Variant::Direct`].
    ///
    /// # Examples
    ///
    /// For small, single domain deployments, direct WKD variant may be
    /// more appropriate than the default:
    ///
    /// ```
    /// use wkd_exporter::{Options, Variant};
    ///
    /// let direct = Options::default().set_variant(Variant::Direct("metacode.biz"));
    /// ```
    #[must_use]
    pub fn set_variant(mut self, variant: Variant<'b>) -> Self {
        self.variant = variant;
        if let Variant::Direct(domain) = self.variant {
            self.set_allowed_domains(vec![domain])
        } else {
            self
        }
    }

    /// Enables or disables append mode.
    ///
    /// When appending is enabled the export will not clear target
    /// files but will rather concatenate incoming certificates to the
    /// ones existing in target directory.
    ///
    /// This could be used to emit multiple certificates for one
    /// e-mail address which is useful for certificate rotation or
    /// storing code-signing certificates along the regular ones.
    ///
    /// # Examples
    ///
    /// Enables append-mode which creates multiple certificates for
    /// one e-mail address:
    ///
    /// ```
    /// use wkd_exporter::Options;
    ///
    /// let append = Options::default().set_append(true);
    /// ```
    #[must_use]
    pub fn set_append(mut self, append: bool) -> Self {
        self.append = append;
        self
    }
}

/// Exports a keyring file (`input`) to a given well known directory.
///
/// # Errors
///
/// If the data is not well-formed OpenPGP packets, then [`Error::Pgp`] is returned.
///
/// # Examples
///
/// ```
/// # fn main() -> testresult::TestResult {
/// use wkd_exporter::{Options, export};
///
/// export(
///     std::fs::File::open("tests/test-cases-default/simple.pgp")?,
///     "/tmp/well-known",
///     &Options::default(),
/// )?;
/// # Ok(()) }
/// ```
#[tracing::instrument(skip(keyring), fields(well_known = ?well_known.as_ref()))]
pub fn export(
    keyring: impl Read,
    well_known: impl AsRef<Path>,
    options: &Options,
) -> Result<(), Error> {
    let openpgpkey = well_known.as_ref().join("openpgpkey");
    std::fs::create_dir_all(&openpgpkey)?;
    let iterator = SignedPublicKey::from_reader_many(keyring)?.0;
    for key in iterator {
        let key = key?;
        export_key(options, &openpgpkey, &key)?;
    }

    Ok(())
}

#[tracing::instrument(fields(key = hex::encode(key.primary_key.fingerprint().as_bytes())))]
fn export_key(
    options: &Options<'_, '_>,
    openpgpkey: &std::path::PathBuf,
    key: &SignedPublicKey,
) -> Result<(), Error> {
    for (encoded_local, domain) in key
        .details
        .users
        .iter()
        // FIXME: rpgp >0.16.0 should make this more compact by reusing the same underlying vector via `into_bytes`:
        // https://github.com/rpgp/rpgp/pull/587/files#diff-5c594c40cbc2d2e401f38bcd423c868e89d9d5cc979fd38081b9a529055b1b0bR78
        .flat_map(|user| String::from_utf8(user.id.id().to_vec()))
        .flat_map(|user| EmailAddress::from_str(&user))
        .map(|email| {
            use sha1::Digest;
            let local_part = email.local_part().to_lowercase();
            let mut digest = sha1::Sha1::new();
            digest.update(local_part.as_bytes());
            let encoded_local = zbase32::encode(&digest.finalize()[..]);
            debug!(?email, encoded_local, "Encoded local part");
            (encoded_local, email.domain().to_string())
        })
        .filter(|(_, domain)| {
            let domain_allowed = options.is_domain_allowed(domain);
            if !domain_allowed {
                debug!(domain, "Skipping domain");
            }
            domain_allowed
        })
    {
        let domain = if let Variant::Advanced = options.variant {
            &openpgpkey.join(&domain)
        } else {
            openpgpkey
        };
        let hu = domain.join("hu");
        std::fs::create_dir_all(&hu)?;

        let policy_file = domain.join("policy");

        trace!(?policy_file, "Making sure the policy file exists");
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&policy_file)?;

        let key_file_path = hu.join(encoded_local);

        let mut key_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(options.append)
            .truncate(!options.append)
            .open(&key_file_path)?;

        info!(?key_file_path, "Writing key to file");
        key.to_writer(&mut key_file)?;
    }
    Ok(())
}
