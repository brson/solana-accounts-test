#![allow(unused)]

mod oldtry;

use solana_program::pubkey::Pubkey;
use variant_count::VariantCount;

pub const MAX_CONSTRAINTS: usize = 10;

pub enum TypeOrConstraint {
    Type(Type),
    Constraint(Constraint),
}

pub enum Type {
    Pubkey,
    PdaString1(&'static str, Pubkey),
}

pub enum Constraint {
    Payer,
    Signer,
    Writable,
    Executable
}

#[repr(usize)]
#[derive(VariantCount)]
pub enum AccountIndex {
    Payer,
    StorageRef,
    Storage,
    NextStorage,
}

#[repr(usize)]
#[derive(VariantCount)]
pub enum KeyIndex {
    Payer,
}

pub const fn make_account_list_constraints() -> [(AccountIndex, TypeOrConstraint); 0] {
    []
}

pub const fn derive_account_list<const N: usize>(
    constraints: &[(AccountIndex, Constraint)],
) -> [Pubkey; N] {
    let mut empty = [Pubkey::new_from_array([0; 32]); N];
    empty
}
