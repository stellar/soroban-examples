use std::{error::Error, io};

use stellar_xdr::curr::{Limited, Limits, OperationBody, ReadXdr, TransactionEnvelope};

fn main() -> Result<(), Box<dyn Error>> {
    let limits = Limits::none();
    let mut limited_read = Limited::new(io::stdin(), limits);
    let mut txe = TransactionEnvelope::read_xdr_base64_to_end(&mut limited_read)?;

    match &mut txe {
        TransactionEnvelope::TxV0(_) => unimplemented!(),
        TransactionEnvelope::TxFeeBump(_) => unimplemented!(),
        TransactionEnvelope::Tx(e) => for op in e.tx.operations.iter_mut() {
            match &op.body {
                OperationBody::InvokeHostFunction(op) => {
                    for auth in op.auth.iter_mut() {
                        println!("Authorizing:\n{}", serde_json::to_string_pretty(&auth)?);
                    }
                },
                _ => {},
            }

        },
    }
    Ok(())
}
