#![allow(unused)]

mod oldtry;

use solana_program::pubkey::Pubkey;


pub const MAX_CONSTRAINTS: usize = 10;

#[repr(usize)]
pub enum AccountIndex {
    PrimaryKey,
}

pub enum Constraint { }

pub const fn make_account_list_constraints() -> [(AccountIndex, Constraint); 0] {
    []
}

