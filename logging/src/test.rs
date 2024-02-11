#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short, testutils::Logs, xdr::Hash, xdr::ScAddress, Address, BytesN, Env, TryFromVal,
};

extern crate std;

#[test]
fn test() {
    let env = Env::default();

    let id_bytes = BytesN::from_array(&env, &[8; 32]);

    let addr: Address =
        Address::try_from_val(&env, &ScAddress::Contract(Hash(id_bytes.to_array()))).unwrap();
    let contract_id = env.register_contract(&addr, Contract);
    let client = ContractClient::new(&env, &contract_id);

    client.hello(&symbol_short!("Dev"));

    let logs = env.logs().all();
    assert_eq!(logs, std::vec!["[Diagnostic Event] contract:CAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQMCJ, topics:[log], data:[\"Hello {}\", Dev]"]);
    std::println!("{}", logs.join("\n"));
}
