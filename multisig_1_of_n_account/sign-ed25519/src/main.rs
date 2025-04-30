//! This stellar-cli plugin receives via stdin a base64 encoded transaction envelope and returns the envelope
//! to stdout with any contract authorizations modified to contain a signature with the key
//! hardcoded in the tool transaction.
//!
//! The plugin requires two environment variables to be set:
//!
//! - `SECRET_KEY` - Set to a 32-byte ed25519 secret key hex-encoded, that will sign any
//! authorization.
//! - `NETWORK_PASSPHRASE` - The network passphrase of the network the invocation is intended to be
//! sent to.
//! - `SIGNATURE_EXPIRATION_LEDGER` - The ledger that the signature will become invalid. Must not
//! be too far in the future.
//!
//! The plugin outputs the modified transaction envelope base64 encoded.

use std::{env, error::Error, io};

use ed25519_dalek::{Keypair, Signer};
use sha2::{Digest, Sha256};
use stellar_xdr::curr::{
    Hash, HashIdPreimage, HashIdPreimageSorobanAuthorization, Limited, Limits, OperationBody,
    ReadXdr, ScBytes, ScMap, ScSymbol, ScVal, SorobanCredentials, TransactionEnvelope, WriteXdr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let secret_key_hex_str = env::var("SECRET_KEY")?;
    let network_passphrase = env::var("NETWORK_PASSPHRASE")?;
    let network_id = Sha256::digest(network_passphrase);
    let signature_expiration_ledger: u32 = env::var("SIGNATURE_EXPIRATION_LEDGER")?.parse()?;

    let secret_key_bytes = hex::decode(secret_key_hex_str)?;
    let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key_bytes)?;
    let public_key = ed25519_dalek::PublicKey::from(&secret_key);
    let keypair = Keypair {
        secret: secret_key,
        public: public_key,
    };
    eprintln!("Public Key: {}", hex::encode(public_key.as_bytes()));

    let limits = Limits::none();
    let mut limited_read = Limited::new(io::stdin(), limits);
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
                                            signature_expiration_ledger,
                                            invocation: auth.root_invocation.clone(),
                                        },
                                    );
                                    let payload_xdr = payload.to_xdr(Limits::none())?;
                                    let payload_hash = Sha256::digest(payload_xdr);
                                    eprintln!("Payload Hash:\n{}", hex::encode(payload_hash));

                                    let signature = keypair.sign(&payload_hash);

                                    creds.signature_expiration_ledger = signature_expiration_ledger;
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
