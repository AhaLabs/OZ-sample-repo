// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.2.0


use soroban_sdk::{Address, contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};
use stellar_fungible::{
    self as fungible, burnable::FungibleBurnable, FungibleToken, mintable::FungibleMintable
};

const OWNER: Symbol = symbol_short!("OWNER");
const MYBOOL: Symbol = symbol_short!("MYBOOL");
const MYENUM: Symbol = symbol_short!("MYENUM");
const STATUS: Symbol = symbol_short!("STATUS");
const TESTKIND: Symbol = symbol_short!("TESTKIND");
const TESTOBJ: Symbol = symbol_short!("TESTOBJ");

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(i8)]
pub enum TestMode {
    Development = 0,
    Testing = 1,
    Production = 2,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestObject {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TestKind {
    Development(String),
    Testing(u128),
    Production(bool, u32),
    Custom(TestObject),
}

#[contract]
pub struct MyToken;


#[contractimpl]
impl MyToken {
    pub fn __constructor(e: &Env, owner: Address, initial_supply: i128, mybool: bool, myenum: TestMode, status: Status, testkind: TestKind, testobj: TestObject) {
        fungible::metadata::set_metadata(e, 18, String::from_str(e, "MyToken"), String::from_str(e, "MTK"));
        fungible::mintable::mint(e, &owner, initial_supply);
        e.storage().instance().set(&OWNER, &owner);
        e.storage().instance().set(&MYBOOL, &mybool);
        e.storage().instance().set(&MYENUM, &myenum);
        e.storage().instance().set(&STATUS, &status);
        e.storage().instance().set(&TESTKIND, &testkind);
        e.storage().instance().set(&TESTOBJ, &testobj);
    }
}

#[contractimpl]
impl FungibleToken for MyToken {
    fn total_supply(e: &Env) -> i128 {
        fungible::total_supply(e)
    }

    fn balance(e: &Env, account: Address) -> i128 {
        fungible::balance(e, &account)
    }

    fn allowance(e: &Env, owner: Address, spender: Address) -> i128 {
        fungible::allowance(e, &owner, &spender)
    }

    fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        fungible::transfer(e, &from, &to, amount);
    }

    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        fungible::transfer_from(e, &spender, &from, &to, amount);
    }

    fn approve(e: &Env, owner: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        fungible::approve(e, &owner, &spender, amount, live_until_ledger);
    }

    fn decimals(e: &Env) -> u32 {
        fungible::metadata::decimals(e)
    }

    fn name(e: &Env) -> String {
        fungible::metadata::name(e)
    }

    fn symbol(e: &Env) -> String {
        fungible::metadata::symbol(e)
    }
}

//
// Extensions
//

#[contractimpl]
impl FungibleBurnable for MyToken {
    fn burn(e: &Env, from: Address, amount: i128) {
        fungible::burnable::burn(e, &from, amount);
    }

    fn burn_from(e: &Env, spender: Address, from: Address, amount: i128) {
        fungible::burnable::burn_from(e, &spender, &from, amount);
    }
}

#[contractimpl]
impl FungibleMintable for MyToken {
    fn mint(e: &Env, account: Address, amount: i128) {
        let owner: Address = e.storage().instance().get(&OWNER).expect("owner should be set");
        owner.require_auth();
        fungible::mintable::mint(e, &account, amount);
    }
}
