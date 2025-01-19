#![no_main]
use libfuzzer_sys::fuzz_target;
use soroban_increment_with_fuzz_contract::{IncrementContract, IncrementContractClient};
use soroban_sdk::{
    testutils::arbitrary::{arbitrary, Arbitrary},
    Env,
};

#[derive(Debug, Arbitrary)]
pub struct Input {
    pub by: u64,
}

fuzz_target!(|input: Input| {
    let env = Env::default();
    let id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &id);

    let mut last: Option<u32> = None;
    for _ in input.by.. {
        match client.try_increment() {
            Ok(Ok(current)) => {
                assert!(Some(current) > last);
                last = Some(current);
            }
            Err(Ok(_)) => {} // Expected error
            Ok(Err(_)) => panic!("success with wrong type returned"),
            Err(Err(_)) => panic!("unrecognised error"),
        }
    }
});
