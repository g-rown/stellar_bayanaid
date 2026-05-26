#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    Address,
    Env,
};

#[contract]
pub struct BayanAid;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Voucher(Address),
    Claimed(Address),
}

#[contractimpl]
impl BayanAid {

    // Initialize contract admin
    pub fn initialize(env: Env, admin: Address) {

        // Prevent double initialization
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // NGO or barangay assigns a relief voucher
    pub fn assign_voucher(
        env: Env,
        admin: Address,
        beneficiary: Address,
        amount: i128,
    ) {

        // Require admin authorization
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        // Only contract admin can assign vouchers
        if admin != stored_admin {
            panic!("unauthorized");
        }

        // Store voucher amount
        env.storage()
            .persistent()
            .set(&DataKey::Voucher(beneficiary), &amount);
    }

    // Beneficiary claims relief aid
    pub fn claim_aid(
        env: Env,
        beneficiary: Address,
    ) -> i128 {

        // Require beneficiary authorization
        beneficiary.require_auth();

        // Check if already claimed
        let already_claimed: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Claimed(beneficiary.clone()))
            .unwrap_or(false);

        if already_claimed {
            panic!("already claimed");
        }

        // Get voucher amount
        let amount: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Voucher(beneficiary.clone()))
            .unwrap_or(0);

        // Ensure voucher exists
        if amount <= 0 {
            panic!("no voucher available");
        }

        // Mark voucher as claimed
        env.storage()
            .persistent()
            .set(&DataKey::Claimed(beneficiary.clone()), &true);

        // Return amount for frontend transfer handling
        amount
    }

    // Check if beneficiary already claimed
    pub fn has_claimed(
        env: Env,
        beneficiary: Address,
    ) -> bool {

        env.storage()
            .persistent()
            .get(&DataKey::Claimed(beneficiary))
            .unwrap_or(false)
    }

    // Read voucher amount
    pub fn get_voucher(
        env: Env,
        beneficiary: Address,
    ) -> i128 {

        env.storage()
            .persistent()
            .get(&DataKey::Voucher(beneficiary))
            .unwrap_or(0)
    }
}