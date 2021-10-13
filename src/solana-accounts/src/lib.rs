#![allow(unused)]

mod oldtry;

use solana_program::pubkey::Pubkey;
use variant_count::VariantCount;

pub const MAX_CONSTRAINTS: usize = 10;

#[derive(Copy, Clone)]
pub enum TypeOrConstraint {
    Type(Type),
    Constraint(Constraint),
}

#[derive(Copy, Clone)]
pub enum Type {
    Pubkey,
    PdaString1(&'static str, Pubkey),
}

#[derive(Copy, Clone)]
pub enum Constraint {
    Payer,
    Signer,
    Writable,
    Executable
}

#[repr(usize)]
#[derive(VariantCount)]
pub enum MyAccountIndex {
    Payer,
    StorageRef,
    Storage,
    NextStorage,
}

#[repr(usize)]
#[derive(VariantCount)]
pub enum MyKeyIndex {
    Payer,
}

pub const fn make_account_list_constraints() -> [(MyAccountIndex, TypeOrConstraint); 0] {
    []
}

pub const fn check_constraint_well_formedness(
    constraints: &[(usize, TypeOrConstraint)],
    num_accounts: usize,
) -> bool {
    let mut constraint_index = 0;

    loop {
        let (account_index, constraint) = constraints[constraint_index];

        constraint_index += 1;
        if constraint_index >= constraints.len() {
            break;
        }
    }

    true
}

pub const fn derive_account_list<const N: usize>(
    constraints: &[(usize, Constraint)],
) -> [Pubkey; N] {
    let mut empty = [Pubkey::new_from_array([0; 32]); N];
    empty
}
