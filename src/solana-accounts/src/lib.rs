#![allow(unused)]

mod oldtry;

use solana_program::pubkey::Pubkey;
use variant_count::VariantCount;

pub type AccountIndex = usize;

#[derive(Copy, Clone)]
pub enum Rule {
    Pubkey,
    Pda {
        static_seeds: &'static [&'static str],
        account_seeds: &'static [AccountIndex],
        program_account: AccountIndex
    },

    Payer,
    Signer,
    Writable,
    Executable
}

#[repr(usize)]
#[derive(VariantCount)]
pub enum StorageAccountIndex {
    Payer,
    Key,
    StorageRef,
    Storage,
    NextStorage,
    StorageProgram,
}

pub const fn make_my_account_rules() -> [(StorageAccountIndex, Rule); 5] {
    [
        (StorageAccountIndex::Payer, Rule::Pubkey),
        (StorageAccountIndex::Payer, Rule::Signer),
        (StorageAccountIndex::Key, Rule::Pubkey),
        (StorageAccountIndex::StorageRef, Rule::Pda {
            static_seeds: &["storage-ref"],
            account_seeds: &[
                StorageAccountIndex::Payer as _,
                StorageAccountIndex::Key as _,
            ],
            program_account: StorageAccountIndex::StorageProgram as _
        }),
        (StorageAccountIndex::Key, Rule::Pubkey),
    ]
}

pub const fn check_rules_well_formedness(
    rules: &[(AccountIndex, Rule)],
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

pub fn verify_accounts(
    accounts: &[Pubkey],
    rules: &[(AccountIndex, Rule)],
) -> bool {
    panic!()
}

pub const fn derive_account_list<const N: usize>(
    rules: &[(AccountIndex, Rule)],
) -> [Pubkey; N] {
    let mut empty = [Pubkey::new_from_array([0; 32]); N];
    empty
}

impl Rule {
    pub const fn is_type(&self) -> bool { false }
    pub const fn is_constraint(&self) -> bool { false }
}
