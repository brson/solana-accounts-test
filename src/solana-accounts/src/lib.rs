#![allow(unused)]

use solana_program::pubkey::Pubkey as SolanaPubkey;


pub type Pubkey = SolanaPubkey;

pub struct Signer<K>(K) where K: Account;

pub struct Payer<K>(K) where K: Account;

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

    fn signer(&self) -> bool { self.0.signer() }
    fn writable(&self) -> bool { self.0.writable() }
    fn executable(&self) -> bool { self.0.executable() }

    fn role_payer(&self) -> bool { true }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub trait Accounts: Sized {
    fn from_pubkey_list(keys: &[SolanaPubkey]) -> Result<Self, Error> {
        panic!()
    }
    fn make_constraints(&self) -> AccountConstraints {
        panic!()
    }
}

pub struct AccountConstraints(Vec<AccountConstraint>);

pub struct AccountConstraint;

pub struct AppInstructionAccounts {
    pub payer: Payer<Pubkey>,
    pub lib_accounts: LibInstructionAccounts,
}

pub struct LibInstructionAccounts {
}

