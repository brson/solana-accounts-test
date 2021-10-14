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

pub const fn make_my_account_list_constraints() -> [(MyAccountIndex, TypeOrConstraint); 1] {
    [
        (MyAccountIndex::Payer, TypeOrConstraint::Type(Type::Pubkey)),
    ]
}

pub const fn check_constraint_well_formedness(
    constraints: &[(usize, TypeOrConstraint)],
    num_accounts: usize,
) -> bool {
    let mut constraint_index = 0;
    let mut current_account_index = 0;

    loop {
        let (account_index, constraint) = constraints[constraint_index];

        if constraint_index == 0 {
            if account_index != 0 {
                return false;
            }
        }

        if account_index > current_account_index + 1 {
            return false;
        }

        if account_index == current_account_index + 1 {
            match constraint {
                TypeOrConstraint::Type(_) => {
                }
                TypeOrConstraint::Constraint(_) => {
                    return false;
                }
            }
        }

        if account_index == current_account_index {
            match constraint {
                TypeOrConstraint::Type(_) => {
                    return false;
                }
                TypeOrConstraint::Constraint(_) => {
                }
            }
        }

        current_account_index = account_index;

        constraint_index += 1;
        if constraint_index >= constraints.len() {
            break;
        }
    }

    true
}

pub fn derive_account_list<const N: usize>(
    constraints: &[(usize, TypeOrConstraint)],
) -> [Pubkey; N] {
    let mut empty = [Pubkey::new_from_array([0; 32]); N];
    empty
}
