// This is a *pseudocode* conceptual example for host-centric Soroban auth.
// It performs an atomic token swap and makes it simple to build all the necessary
// signatures to perform it.
struct AtomicSwap;

// This is a custom role description for the `swap` operation.
// Every role just corresponds to the integer index in the outer `signatures`
// vec that we pass to the `swap` call.
// One or more signers may share the same role. It's up to the contract whether
// more than one signer for each role is supported.
pub enum SwapRole {
    Seller,
    Buyer,
}

#[contractimpl]
impl AtomicSwap {
    // This function swaps the `sell_amount` of `sell_token_id` from
    // SwapRole::Seller signer with the `buy_amount` of `buy_token_id`
    // from SwapRole::Buyer signer.
    // Seller and buyer roles must include the `xfer` signatures to pass to the
    // respective tokens. That allows to skip `approve` operations and perform
    // the swap atomically.
    fn swap(
        e: Env,
        sell_token_id: BytesN<32>,
        sell_amount: BigInt,
        buy_token_id: BytesN<32>,
        buy_amount: BigInt,
    ) {
        // `get_signer` here is just a convenience wrapper around a host
        // function `get_signers` that enforce the result vector to contain
        // exactly one signer.
        // See `get_signers` comment in the previous example for details.
        let seller: Identifier = e.get_signer(SwapRole::Seller as u32);
        let buyer: Identifier = e.get_signer(SwapRole::Buyer as u32);

        let sell_token = token::Client::new(&e, sell_token_id);
        let buy_token = token::Client::new(&e, buy_token_id);

        let contract_id = Identifier::Contract(e.get_current_contract().into());
        // `with_role_signatures` maps the role space of the current contract
        // (Seller/Buyer here) to the role of the contract being called (token
        // needs only the default (0) role).
        // As described in the previous example, `get_signers` function finds
        // the proper signature by the call stack,
        // hence we don't need to remove the signatures that are not known
        // to the called contract.
        sell_token
            .with_role_signatures(vec![&e, SwapRole::Seller as u32])
            .xfer(/* to */ &contract_id, /* amount */ &sell_amount);
        buy_token
            .with_role_signatures(vec![&e, SwapRole::Buyer as u32])
            .xfer(/* to */ &contract_id, /* amount */ &buy_amount);

        // Another way to call the contract is via explicitly providing the
        // special `signatures` argument. That would override `signatures` for
        // this call only.
        // We may provide some helpers for simple cases to avoid explicitly
        // building a 3-d vec for this (most of the time we would only want to
        // call the inner contracts with the only `Invoker`).
        sell_token
            .with_signatures(vec![&e, vec![&e, vec![&e, &Signature::Invoker]]])
            .xfer(/* to */ &buyer, /* amount */ &sell_amount);
        buy_token
            .with_signatures(vec![&e, vec![&e, vec![&e, &Signature::Invoker]]])
            .xfer(/* to */ &seller, /* amount */ &buy_amount);
    }
}

// This is a custom implementation of `SignaturePayloadTrait`.
#[contractimpl]
impl SignaturePayloadTrait for AtomicSwap {
    fn _payload(
        e: Env,
        role: u32,
        function_name: Symbol,
        args: Vec<RawVal>,
    ) -> Vec<SignaturePayload> {
        match function {
            symbol!("swap") => match scope.try_into::<SwapRole>().unwrap() {
                SwapRole::Seller => swap_seller_payload(&e, function_name, args),
                // Buyer and seller signatures are symmetric in this example,
                // hence all the users may just sign the `Seller` role payload
                // and then the contract invoker can just pass the role signatures
                // to `swap` invocation in any order (they just need to make
                // sure that the contract is called from the `Seller` perspective).
                //
                // Note: there is some merit to include role into the signature
                // payload in case if multiple roles with the same args are
                // needed. OTOH it's more flexible to just include role as
                // `salt` in such cases, so that symmetric cases like this one
                // are still possible to implement. Probably either way would
                // work.
                SwapSignatureScope::Buyer => swap_seller_payload(
                    &e,
                    function_name,
                    vec![&e, args[2], args[3], args[0], args[1]],
                ),
            },
            _ => panic!(),
        }
    }
}

fn swap_seller_payload(e: &Env, function_name: Symbol, args: Vec<RawVal>) -> Vec<SignaturePayload> {
    // Start with the regular payload for the `swap` invocation.
    let mut payload = vec![
        e,
        e.signature_payload_for_curr_contract(
            function_name.clone(),
            args,
            /* use_nonce */ true,
            /* salt */ None,
        ),
    ];
    let token = token::Client::new(&e, args[0].into());
    // This is another helper for building payloads. It takes the payload of
    // the inner contract call, appends `(e.get_current_contract(), function_name)`
    // to the call stacks of the resulting payloads and appends it to the
    // provided output payload. This way the final `payload` method is guaranteed
    // to have the payloads with the correct call stacks as long as every inner
    // contract uses `add_signature_payload_for_contract_call`.
    //
    // Optimization note: The tricky part here is that the full payload makes sense
    // only for the signing calls of `_payload`. During the invocation calls
    // we are only interested in payload for the current  contract call and in
    // call stacks for inner contract calls. If that's a big performance issue,
    // and we can't optimize it, then we'd probably need to drop the nice UX of
    // automatically getting payload from call stacks and instead let the user
    // specify the ids of signatures manually.
    e.add_signature_payload_for_contract_call(
        &mut payload,
        function_name,
        &token,
        DEFAULT_SIGNATURE_ROLE,
        &symbol!("xfer"),
        &vec![&e,
        /* to */ Identifier::Contract(e.get_current_contract().into()).into()
        /* amount */ args[1]],
    );

    payload
}
