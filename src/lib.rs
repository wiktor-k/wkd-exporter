use std::{fs::OpenOptions, io::Read, path::PathBuf, str::FromStr};

use email_address::EmailAddress;
use pgp::{ser::Serialize, Deserializable, SignedPublicKey};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    pub well_known: PathBuf,
}

pub fn run(args: Args, input: impl Read) -> testresult::TestResult {
    let openpgpkey = args.well_known.join("openpgpkey");
    std::fs::create_dir_all(&openpgpkey)?;
    let iterator = SignedPublicKey::from_reader_many(input)?.0;
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
