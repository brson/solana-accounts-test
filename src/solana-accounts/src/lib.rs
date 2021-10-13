#![allow(unused)]

mod oldtry;

use solana_program::pubkey::Pubkey;
use variant_count::VariantCount;

pub const MAX_CONSTRAINTS: usize = 10;

#[repr(usize)]
#[derive(VariantCount)]
pub enum AccountIndex {
    PrimaryKey,
}

pub enum Constraint { }

pub const fn make_account_list_constraints() -> [(AccountIndex, Constraint); 0] {
    []
}

pub const fn derive_account_list<const N: usize>() -> [Pubkey; N] {
    let mut empty = [Pubkey::new_from_array([0; 32]); N];
    empty
}
