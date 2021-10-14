#![allow(unused)]

mod oldtry;

use solana_program::pubkey::Pubkey;
use variant_count::VariantCount;

#[derive(Copy, Clone)]
pub enum Rule {
    Pubkey,
    PdaString1(&'static str, Pubkey),

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

pub const fn make_my_account_rules() -> [(MyAccountIndex, Rule); 1] {
    [
        (MyAccountIndex::Payer, Rule::Pubkey),
    ]
}

pub const fn check_rules_well_formedness(
    rules: &[(usize, Rule)],
) -> bool {
    let mut rule_index = 0;
    let mut current_account_index = 0;

    loop {
        let (account_index, rule) = rules[rule_index];

        if rule_index == 0 {
            if account_index != 0 {
                return false;
            }
        }

        if account_index > current_account_index + 1 {
            return false;
        }

        if account_index == current_account_index + 1 {
            if rule.is_constraint() {
                return false;
            }
        }

        if account_index == current_account_index {
            if rule.is_type() {
                return false;
            }
        }

        current_account_index = account_index;

        rule_index += 1;
        if rule_index >= rules.len() {
            break;
        }
    }

    true
}

pub const fn derive_account_list<const N: usize>(
    rules: &[(usize, Rule)],
) -> [Pubkey; N] {
    let mut empty = [Pubkey::new_from_array([0; 32]); N];
    empty
}

impl Rule {
    pub const fn is_type(&self) -> bool { false }
    pub const fn is_constraint(&self) -> bool { false }
}
