# Soroban PRNG Example <!-- omit in toc -->

This example demonstrates how to use Soroban's Pseudorandom Number Generator (PRNG) API, and how to implement a **commit-reveal scheme** to secure random choices against "blind-rerun" attacks.

## Overview

Soroban provides a deterministic on-chain PRNG accessible via `env.prng()`. The host maintains an internal PRNG state seeded from protocol entropy and contract contexts.

Key PRNG operations demonstrated in this contract:

- `env.prng().gen_range(min..=max)`: Draws a pseudorandom integer within an inclusive range.
- `env.prng().gen::<T>()`: Generates a pseudorandom value of type `T` (e.g. `BytesN<32>`).
- `env.prng().shuffle(&mut vec)`: Performs an in-place Fisher-Yates shuffle on a collection.
- `env.prng().seed(bytes)`: Reseeds the contract's PRNG with caller-supplied entropy.

## Security: Guarding Against "Blind-Rerun" Attacks

### The Vulnerability

If a contract makes an economically consequential pseudorandom choice within a single transaction (e.g., a coin flip or lottery):

```rust
// VULNERABLE PATTERN: single-turn random choice
pub fn play(env: Env) -> bool {
    let won = env.prng().gen_range(1..=2) == 1;
    if won {
        distribute_prize(&env);
    }
    won
}
```

An adversarial smart contract can wrap the call inside a loop with `try_call`:

1. The attacker contract invokes `play()` via `env.try_invoke_contract(...)`.
2. If `play()` returns a losing outcome, the attacker contract immediately panics or aborts the sub-invocation, rolling back state changes.
3. The PRNG state in the sub-contract advances upon failure (to prevent predictable seed exploitation), but the attacker suffers no economic loss other than resource/gas fees.
4. The attacker repeats this in a loop until a winning result is returned.

### The Mitigation: Commit-Reveal Scheme

To eliminate this vulnerability, random choices should use a two-phase **commit-reveal scheme**:

1. **Commit Phase (`commit`)**:
   - The user authorizes and registers their participation.
   - The contract generates a private 32-byte seed from the PRNG (`env.prng().gen()`) and records a minimum future ledger sequence (`env.ledger().sequence() + 1`).
   - No winner or outcome is determined in this transaction. Even if an attacker rolls back their transaction, they gain zero information on what the final outcome will be.
2. **Reveal Phase (`reveal`)**:
   - In a subsequent ledger (`current_ledger >= reveal_ledger`), the contract or player reveals the outcome.
   - The contract reseeds its PRNG using the previously committed seed (`env.prng().seed(...)`) and computes the outcome.
   - Because the seed was finalized in an earlier ledger, the outcome is completely deterministic with respect to the commitment. The attacker cannot bias or rerun the decision.

## Code Structure

- `src/lib.rs`: The smart contract definition:
  - `gen_number`: Basic range-based random number generation.
  - `shuffle_deck`: In-place vector shuffle.
  - `commit`: Stores an unrevealed random commitment locked to `current_ledger + 1`.
  - `reveal`: Verifies the ledger sequence, reseeds the PRNG, and reveals the random outcome.
  - `get_commitment`: Queries commitment state.
- `src/test.rs`: Unit tests verifying basic generation, shuffle integrity, commit-reveal phase requirements, and determinism.

## How to Run the Tests

To build and run tests:

```bash
cargo test
```
