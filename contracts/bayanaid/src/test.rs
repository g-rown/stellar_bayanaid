#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _},
    Address,
    Env,
};

use crate::BayanAid;

#[test]
fn test_happy_path_claim() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanAid);

    let client = BayanAidClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let beneficiary = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin);

    // Assign relief voucher
    client.assign_voucher(&admin, &beneficiary, &1000);

    // Claim aid
    let amount = client.claim_aid(&beneficiary);

    assert_eq!(amount, 1000);
}

#[test]
#[should_panic]
fn test_duplicate_claim() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanAid);

    let client = BayanAidClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let beneficiary = Address::generate(&env);

    client.initialize(&admin);

    client.assign_voucher(&admin, &beneficiary, &500);

    // First claim
    client.claim_aid(&beneficiary);

    // Second claim should fail
    client.claim_aid(&beneficiary);
}

#[test]
fn test_state_verification() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanAid);

    let client = BayanAidClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let beneficiary = Address::generate(&env);

    client.initialize(&admin);

    client.assign_voucher(&admin, &beneficiary, &800);

    client.claim_aid(&beneficiary);

    // Verify storage updated
    let claimed = client.has_claimed(&beneficiary);

    assert_eq!(claimed, true);
}

#[test]
#[should_panic]
fn test_unauthorized_assign() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanAid);

    let client = BayanAidClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let attacker = Address::generate(&env);
    let beneficiary = Address::generate(&env);

    client.initialize(&admin);

    // Unauthorized wallet tries assigning voucher
    client.assign_voucher(&attacker, &beneficiary, &100);
}

#[test]
fn test_get_voucher() {

    let env = Env::default();

    let contract_id = env.register_contract(None, BayanAid);

    let client = BayanAidClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let beneficiary = Address::generate(&env);

    client.initialize(&admin);

    client.assign_voucher(&admin, &beneficiary, &300);

    let amount = client.get_voucher(&beneficiary);

    assert_eq!(amount, 300);
}