// This is a *pseudocode* conceptual example for host-centric Soroban auth.
// It does a simple authenticated operation for an arbitrary number of signers
// at once.
// The goal here is to prototype how a relatively complex auth could be
// auto-generated with macros, so that no authentication code needs to be
// written at all.

struct SimpleMultisig;

#[contracttype]
pub enum DataKey {
    HasFlag(Identifier),
}

#[contractimpl]
// Magic macro that automatically implements the default `SignaturePayloadTrait`
// for a contract while following the method annotations.
#[default_signatures]
impl SimpleMultisig {
    // #[signed] means that all the arguments of this method should be signed
    // (in order). By default the external nonce will be attached to the
    // signature.
    // #[signed_no_nonce] could be introduced to support a nonce-less
    // signature.
    #[signed]
    // When invoking a contract function that has signatures a special
    // Vec<Vec<Vec<Signature>>> `signatures` argument needs to be provided.
    // This argument is meant to be opaque for the most use-cases and should be
    // built via SDK/host utilities. It contains all the signatures for handling
    // the authentication on the host side. Outer vec is roles, middle one
    // is signers (for the role) and the inner one is payloads for the role signed
    // by the corresponding signer. All the inner signatures must have the same
    // identifier.
    //
    // Before invoking any contract function, env calls the
    // `_payload` function for every role in the `signatures` vector and stores
    // the payloads to be used for each role (we refer to this as `payloads` below).
    // Now that we have the payloads and signatures, we can do some basic validation
    // and make sure that the signature set is valid (but not yet verified).
    fn set_flag(e: Env, flag: bool) {
        // `get_signers` call gets the authenticated signers for the requested
        // role and the current call stack. This does the following (on the
        // host side):
        // 1. Get the proper payload id for the requested role.
        //    The payload id is found by the current call stack, i.e. among all 
        //    the `payloads` for the current role we find the id of the payload
        //    that has call stack equal to the current call stack.
        // 2. Verify all the signatures in the `[role][..][payload_id]` slice of
        //    `signatures` (panic/error if unsuccessful). If nonce is needed, 
        //    read and consume host-managed nonce and attach it to the payload
        //    that's being verified.
        // 3. Return the flattened identifiers of signatures in slice.
        let signers: Vec<Identifier> = e.get_signers(DEFAULT_SIGNATURE_ROLE /* == 0 */);
        for signer in signers.iter() {
            e.data().set(DataKey::HasFlag(signer.unwrap()), flag);
        }
    }

    // If there is no #[signed] annotation, then no signatures are required for this.
    // This could be an explicit annotation too.
    fn get_flag(e: Env, id: Identifier) -> bool {
        e.data().get(DataKey::HasFlag(id))
    }
}

// [default_signature] would expand to something like the following
// (all the code below):
#[contractimpl]
impl SignaturePayloadTrait for SimpleMultisigIncrement {
    // This is the function that provides payloads both for signing and
    // verification. For an existing contract function and args it returns a
    // vector of payloads for the requested role.
    // Every payload is expected to correspond to a unique call stack, i.e.
    // calling the same contract function for the same role twice is not supported.
    // The returned payload is *not* yet what needs to be signed. For each signer
    // that belongs to the role and for each payload, the wallet has to get the
    // nonce (when `use_nonce` is true) and only then sign the whole payload.
    // Nonce management happens on the host side. Nonce key is (contract_id, identifier).
    fn _payload(
        e: Env,
        role: u32,
        function_name: Symbol,
        args: Vec<RawVal>,
    ) -> Vec<SignaturePayload> {
        match function {
            symbol!("set_flag") => match role {
                // `signature_payload_for_curr_contract` function builds a
                // payload by injecting the current env parameters. Payload
                // consists of:
                // - network passphrase (e.ledger().network_passphrase())
                // - call stack (including the current function).
                //   Here it's [(e.get_current_contract(), function_name)])
                // - arguments of the top function of stack
                // - `use_nonce` flag that tells whether host-managed nonce needs
                //   to be used for the top contract of stack
                // - optional user-provided salt
                0 => vec![
                    &e,
                    e.signature_payload_for_curr_contract(
                        function_name,
                        args,
                        /* use_nonce */ true,
                        /* salt */ None,
                    ),
                ],
                _ => panic!(),
            },
            // No signatures needed here - just return an empty vector (probably
            // we could just panic here instead)
            symbol!("get_flag") => vec![&e],
            _ => panic!(),
        }
    }
}
