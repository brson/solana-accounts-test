#![allow(unused)]

use solana_program::pubkey::Pubkey as SolanaPubkey;


pub type Pubkey = SolanaPubkey;

pub struct Signer<K>(K) where K: Account;

pub struct Payer<K>(K) where K: Account;

pub struct Ref<K>(K) where K: Account;

pub struct Obj<K>(K) where K: Account;

pub trait Account {
    fn pubkey(&self) -> SolanaPubkey;
    fn signer(&self) -> bool;
    fn writable(&self) -> bool;
    fn owner(&self) -> Option<SolanaPubkey>;
    fn executable(&self) -> bool;

    fn role_payer(&self) -> bool;
}

impl Account for Pubkey {
    fn pubkey(&self) -> SolanaPubkey { *self }
    fn owner(&self) -> Option<SolanaPubkey> { None }

    fn signer(&self) -> bool { false }
    fn writable(&self) -> bool { false }
    fn executable(&self) -> bool { false }

    fn role_payer(&self) -> bool { false }
}

impl<K> Account for Signer<K> where K: Account {
    fn pubkey(&self) -> SolanaPubkey { self.0.pubkey() }
    fn owner(&self) -> Option<SolanaPubkey> { self.0.owner() }

    fn signer(&self) -> bool { true }
    fn writable(&self) -> bool { self.0.writable() }
    fn executable(&self) -> bool { self.0.executable() }

    fn role_payer(&self) -> bool { self.0.role_payer() }
}

impl<K> Account for Payer<K> where K: Account {
    fn pubkey(&self) -> SolanaPubkey { self.0.pubkey() }
    fn owner(&self) -> Option<SolanaPubkey> { self.0.owner() }

    fn signer(&self) -> bool { true } /* payer is a signer */
    fn writable(&self) -> bool { self.0.writable() }
    fn executable(&self) -> bool { self.0.executable() }

    fn role_payer(&self) -> bool { true }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub trait Accounts: Sized {
    fn from_payer(payer: SolanaPubkey) -> Self {
        panic!()
    }
    fn from_pubkey_list(keys: &[SolanaPubkey]) -> Result<Self, Error> {
        let constraints = Self::account_list_constraints();
        constraints.verify(keys)?;
        Ok(Self::from_payer(keys[constraints.payer_index()]))
    }
    fn account_list_constraints() -> AccountListConstraints {
        panic!()
    }
}

pub struct AccountListConstraints(Vec<AccountListConstraint>);

pub struct AccountListConstraint;

impl AccountListConstraints {
    pub fn verify(&self, keys: &[SolanaPubkey]) -> Result<(), Error> {
        panic!()
    }

    pub fn payer_index(&self) -> usize { 0 }
}


mod ex {
    use super::*;

    pub struct SetInstructionAccounts {
        pub payer: Payer<Pubkey>,
        pub storage_ref: Ref<Pubkey>,
        pub storage: Obj<Pubkey>,
        pub next_storage: Pubkey,
    }

    pub struct LibInstructionAccounts {
    }

}
