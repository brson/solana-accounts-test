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

pub struct AccountsDesc {
    pub payer: SolanaPubkey,
}



pub struct AppInstructionAccounts {
    pub payer: Payer<Pubkey>,
    pub lib_accounts: LibInstructionAccounts,
}

pub struct LibInstructionAccounts {
}
