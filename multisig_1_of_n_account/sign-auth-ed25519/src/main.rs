//! This stellar-cli plugin receives via stdin a base64 encoded transaction envelope and returns the envelope
//! to stdout with any contract authorizations modified to contain a signature with the key
//! hardcoded in the tool transaction.
//!
//! The plugin outputs the modified transaction envelope base64 encoded, ready to be re-simulated
//! and submitted.

use std::{
    error::Error,
    fmt::Debug,
    io::{self, Read},
};

use clap::Parser;
use ed25519_dalek::{Keypair, Signer};
use sha2::{Digest, Sha256};
use stellar_xdr::curr::{
    Hash, HashIdPreimage, HashIdPreimageSorobanAuthorization, Limited, Limits, OperationBody,
    ReadXdr, ScBytes, ScMap, ScSymbol, ScVal, SorobanCredentials, TransactionEnvelope, WriteXdr,
};

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cli {
    #[arg(long)]
    secret_key: String,
    #[arg(long, default_value = "Test SDF Network ; September 2015")]
    network_passphrase: String,
    #[arg(long)]
    signature_expiration_ledger: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let network_id = Sha256::digest(cli.network_passphrase);

    let secret_key_bytes = hex::decode(cli.secret_key)?;
    let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key_bytes)?;
    let public_key = ed25519_dalek::PublicKey::from(&secret_key);
    let keypair = Keypair {
        secret: secret_key,
        public: public_key,
    };
    eprintln!("Public Key: {}", hex::encode(public_key.as_bytes()));

    let limits = Limits::none();
    let mut limited_read = Limited::new(SkipWhitespace::new(io::stdin()), limits);
    let mut txe = TransactionEnvelope::read_xdr_base64_to_end(&mut limited_read)?;

    match &mut txe {
        TransactionEnvelope::TxV0(_) => unimplemented!(),
        TransactionEnvelope::TxFeeBump(_) => unimplemented!(),
        TransactionEnvelope::Tx(e) => {
            for op in e.tx.operations.iter_mut() {
                match &mut op.body {
                    OperationBody::InvokeHostFunction(op) => {
                        for auth in op.auth.iter_mut() {
                            eprintln!("Authorizing:\n{}", serde_json::to_string_pretty(&auth)?);
                            match &mut auth.credentials {
                                SorobanCredentials::Address(creds) => {
                                    let payload = HashIdPreimage::SorobanAuthorization(
                                        HashIdPreimageSorobanAuthorization {
                                            network_id: Hash(network_id.try_into()?),
                                            nonce: creds.nonce,
                                            signature_expiration_ledger: cli
                                                .signature_expiration_ledger,
                                            invocation: auth.root_invocation.clone(),
                                        },
                                    );
                                    let payload_xdr = payload.to_xdr(Limits::none())?;
                                    let payload_hash = Sha256::digest(payload_xdr);
                                    eprintln!("Payload Hash:\n{}", hex::encode(payload_hash));

                                    let signature = keypair.sign(&payload_hash);

                                    creds.signature_expiration_ledger =
                                        cli.signature_expiration_ledger;
                                    creds.signature = ScVal::Map(Some(ScMap::sorted_from([
                                        (
                                            ScVal::Symbol(ScSymbol("public_key".try_into()?)),
                                            ScVal::Bytes(ScBytes(
                                                public_key.as_bytes().try_into()?,
                                            )),
                                        ),
                                        (
                                            ScVal::Symbol(ScSymbol("signature".try_into()?)),
                                            ScVal::Bytes(ScBytes(signature.to_bytes().try_into()?)),
                                        ),
                                    ])?));

                                    eprintln!(
                                        "Authorized:\n{}",
                                        serde_json::to_string_pretty(&auth)?
                                    );
                                }
                                _ => continue,
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    println!("{}", txe.to_xdr_base64(Limits::none())?);

    Ok(())
}

pub struct SkipWhitespace<R: Read> {
    pub inner: R,
}

impl<R: Read> SkipWhitespace<R> {
    pub fn new(inner: R) -> Self {
        SkipWhitespace { inner }
    }
}

impl<R: Read> Read for SkipWhitespace<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;

        let mut written = 0;
        for read in 0..n {
            if !buf[read].is_ascii_whitespace() {
                buf[written] = buf[read];
                written += 1;
            }
        }

        Ok(written)
    }
}
