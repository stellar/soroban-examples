# Receipt anchor

A minimal example of committing to a contract invocation so that a third party
can later confirm — from public data alone — that a specific invocation
produced a specific result.

## What it does

`anchor` hashes a fixed tuple of invocation facts and stores the resulting
receipt under the caller's address:

```text
commitment = sha256( "receipt-anchor/1"
                   ++ xdr(target)
                   ++ xdr(fn_name)
                   ++ sha256(xdr(args))
                   ++ sha256(xdr(result))
                   ++ xdr(ledger) )
```

| function | what it is for |
| --- | --- |
| `anchor(caller, target, fn_name, args, result, ledger)` | require the caller's authorization, compute the commitment, store the receipt, return the commitment |
| `commitment(target, fn_name, args, result, ledger)` | recompute the same 32 bytes without writing anything |
| `verify(receipt, args, result)` | recompute and compare against a stored receipt |
| `receipt(caller, index)` / `count(caller)` | read a caller's receipts in anchoring order |

The span of the commit is the invocation itself: the contract, the function,
the arguments, the result, and the ledger. Change any one of them and the
commitment changes.

## What it does not do

- It does not decide whether a call **should** happen. There is no policy, no
  allowlist, no deadline check, no size limit. Those belong to whoever is
  authorizing the call, not to the layer recording that it happened.
- It does not move funds, hold keys, or grant authority.
- It says nothing about whether the invocation was *correct*, only that these
  are the facts of the invocation that was anchored.

## Layout

```text
src/lib.rs      the contract
src/test.rs     the tests
Cargo.toml      soroban-sdk 28, matching this repository's other examples
```

## Run

```sh
make test     # cargo test
make build    # stellar contract build
```

`make build` needs `stellar-cli` 25.2.0 or newer: soroban-sdk 28 refuses to build
for `wasm32-unknown-unknown`, and the CLI builds for `wasm32v1-none` and puts the
contract at `target/wasm32v1-none/release/soroban_receipt_anchor_contract.wasm`.

Verified: `cargo test` reports 9 passed, and the release build exports the five
entry points (`anchor`, `commitment`, `verify`, `count`, `receipt`).

## Verifying a receipt off-chain

A verifier that has the receipt does not need to call the contract:

1. Take `target`, `fn_name`, `args`, `result` and `ledger` from the invocation
   being claimed.
2. Encode `target`, `fn_name` and `ledger` as XDR `ScVal`s, and take SHA-256 of
   the XDR of `args` and of `result` separately.
3. Concatenate `"receipt-anchor/1"`, the three XDR encodings in the order above,
   and the two 32-byte component hashes, then SHA-256 the result.
4. Compare with the receipt's `commitment`.

Every component is either fixed-width or hash-fixed-width, so no two different
tuples can produce the same preimage by shifting a boundary. The stored
`args_hash` and `result_hash` are there so a verifier can report *which*
component disagreed instead of only that something did.

## Tests

Nine, and the tamper cases are the point: changing the args, the result, the
ledger, the target, or the function name each breaks verification on its own,
so a receipt that passes is not passing for an incidental reason. Two also
check the content-addressed property — the same invocation anchored by two
different callers produces the same commitment, and neither caller can occupy
the other's index. The last one calls `anchor` without mocking authorization
and expects it to fail, so the `require_auth` gate is tested rather than
assumed.

## Limits

- Storage entries are written to persistent storage without TTL management;
  a production contract would `extend_ttl` on write and read.
- `anchor` is called by the party making the claim. It records that the caller
  asserted these facts at this ledger, not that an independent observer
  witnessed the invocation. Binding a receipt to a host-produced execution
  result is a further step, and a larger one.
- The commitment covers the arguments as a whole list; it does not say which
  argument matters to the caller. A policy that only cares about one field
  would hash that field, not the list.
