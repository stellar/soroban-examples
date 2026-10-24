#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    vec, Env,
};

#[test]
fn test_basic_prng_and_shuffle() {
    let env = Env::default();
    let contract_id = env.register(PrngContract, ());
    let client = PrngContractClient::new(&env, &contract_id);

    // Generate number within range [10, 50].
    for _ in 0..20 {
        let n = client.gen_number(&10, &50);
        assert!(n >= 10 && n <= 50);
    }

    // Invalid range should return Error::InvalidRange.
    let err = client.try_gen_number(&50, &10).unwrap_err();
    assert_eq!(err, Ok(Error::InvalidRange));

    // Test shuffle.
    let original = vec![&env, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let shuffled = client.shuffle_deck(&original);
    assert_eq!(shuffled.len(), original.len());

    // Verify all original elements are present in shuffled output.
    for i in 1..=10u32 {
        assert!(shuffled.contains(i));
    }
}

#[test]
fn test_commit_reveal_workflow() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PrngContract, ());
    let client = PrngContractClient::new(&env, &contract_id);
    let player = Address::generate(&env);

    // 1. Reveal before commit should fail with NotCommitted.
    let early_err = client.try_reveal(&player, &1, &100).unwrap_err();
    assert_eq!(early_err, Ok(Error::NotCommitted));

    // 2. Commit a new random decision.
    let initial_ledger = env.ledger().sequence();
    client.commit(&player);

    let commitment = client.get_commitment(&player);
    assert_eq!(commitment.player, player);
    assert_eq!(commitment.reveal_ledger, initial_ledger + 1);
    assert_eq!(commitment.revealed_number, None);

    // 3. Attempting to commit again before revealing should fail.
    let dup_err = client.try_commit(&player).unwrap_err();
    assert_eq!(dup_err, Ok(Error::AlreadyCommitted));

    // 4. Attempting to reveal in the same ledger should fail with RevealTooEarly.
    let too_early_err = client.try_reveal(&player, &1, &100).unwrap_err();
    assert_eq!(too_early_err, Ok(Error::RevealTooEarly));

    // 5. Advance the ledger sequence.
    env.ledger().with_mut(|li| {
        li.sequence_number += 1;
    });

    // 6. Reveal should now succeed and return a number in [1, 100].
    let result = client.reveal(&player, &1, &100);
    assert!(result >= 1 && result <= 100);

    let updated = client.get_commitment(&player);
    assert_eq!(updated.revealed_number, Some(result));

    // 7. Second reveal should fail with AlreadyRevealed.
    let already_err = client.try_reveal(&player, &1, &100).unwrap_err();
    assert_eq!(already_err, Ok(Error::AlreadyRevealed));

    // 8. After revelation, a new commitment can be started.
    client.commit(&player);
    let new_commitment = client.get_commitment(&player);
    assert_eq!(new_commitment.revealed_number, None);
    assert_eq!(new_commitment.reveal_ledger, env.ledger().sequence() + 1);
}

#[test]
fn test_commit_reveal_resilience_to_blind_rerun() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PrngContract, ());
    let client = PrngContractClient::new(&env, &contract_id);
    let player = Address::generate(&env);

    client.commit(&player);
    let commitment = client.get_commitment(&player);

    // Advance ledger so reveal is valid.
    env.ledger().with_mut(|li| {
        li.sequence_number += 1;
    });

    // In a commit-reveal design, the outcome is deterministically sealed once the seed is chosen.
    // We verify that re-running with the committed seed inside contract context produces the exact same outcome.
    let outcome = client.reveal(&player, &1, &1_000_000);

    // Reproduce in another contract context using the same seed to confirm determinism.
    let test_env = Env::default();
    let test_contract_id = test_env.register(PrngContract, ());
    let seed_bytes = Bytes::from_slice(&test_env, &commitment.committed_seed.to_array());

    let replicated: u64 = test_env.as_contract(&test_contract_id, || {
        test_env.prng().seed(seed_bytes);
        test_env.prng().gen_range(1..=1_000_000)
    });

    assert_eq!(
        outcome, replicated,
        "Committed seed must deterministically produce the exact same result"
    );
}

#[test]
fn test_blind_rerun_attack_prevented_by_commit_reveal() {
    let env = Env::default();
    env.mock_all_auths();

    let victim_id = env.register(PrngContract, ());
    let victim_client = PrngContractClient::new(&env, &victim_id);
    let player = Address::generate(&env);

    // 1. Attacker cannot inspect or bias outcome at commit time because no outcome exists.
    victim_client.commit(&player);
    let comm = victim_client.get_commitment(&player);
    assert_eq!(comm.revealed_number, None);

    // 2. Advancing to reveal ledger.
    env.ledger().with_mut(|li| {
        li.sequence_number += 1;
    });

    // 3. Reveal produces the number and permanently transitions the state to revealed.
    let revealed = victim_client.reveal(&player, &1, &100);

    // 4. Any attempt to re-invoke reveal to shop for a different random number fails.
    let retry = victim_client.try_reveal(&player, &1, &100).unwrap_err();
    assert_eq!(retry, Ok(Error::AlreadyRevealed));

    // 5. The recorded number in storage remains strictly equal to the original outcome.
    assert_eq!(
        victim_client.get_commitment(&player).revealed_number,
        Some(revealed)
    );
}

#[test]
fn test_multiple_players_isolated() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(PrngContract, ());
    let client = PrngContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    client.commit(&alice);
    client.commit(&bob);

    let comm_alice = client.get_commitment(&alice);
    let comm_bob = client.get_commitment(&bob);

    assert_ne!(
        comm_alice.committed_seed, comm_bob.committed_seed,
        "Distinct commitments should have distinct PRNG seeds"
    );

    env.ledger().with_mut(|li| {
        li.sequence_number += 1;
    });

    let res_alice = client.reveal(&alice, &1, &1000);
    let res_bob = client.reveal(&bob, &1, &1000);

    assert!(res_alice >= 1 && res_alice <= 1000);
    assert!(res_bob >= 1 && res_bob <= 1000);

    assert_eq!(
        client.get_commitment(&alice).revealed_number,
        Some(res_alice)
    );
    assert_eq!(client.get_commitment(&bob).revealed_number, Some(res_bob));
}
