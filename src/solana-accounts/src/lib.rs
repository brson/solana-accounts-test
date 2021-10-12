use solana_program::pubkey::Pubkey as SolanaPubkey;


pub type Pubkey = SolanaPubkey;

pub struct Signer<K>(K) where K: Account;

pub trait Account {
    fn pubkey(&self) -> SolanaPubkey;
    fn signer(&self) -> bool;
    fn writable(&self) -> bool;
    fn owner(&self) -> Option<SolanaPubkey>;
    fn executable(&self) -> bool;
}
