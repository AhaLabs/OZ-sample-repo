#![cfg(test)]

extern crate std;

use soroban_sdk::{ testutils::Address as _, Address, Env, String };

use crate::contract::{ MyToken, MyTokenClient };

#[test]
fn initial_state() {
    let env = Env::default();

    let contract_addr = env.register(MyToken, (Address::generate(&env),));
    let client = MyTokenClient::new(&env, &contract_addr);

    assert_eq!(client.name(), String::from_str(&env, "MyToken"));
}

// Add more tests bellow
