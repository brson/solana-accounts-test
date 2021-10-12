use solana_program::pubkey::Pubkey as SolanaPubkey;


pub type Pubkey = SolanaPubkey;

pub struct Signer<K>(K) where K: Accounts;

pub trait Accounts {
    fn pubkeys(&self) -> Vec<Pubkey>;
    fn signers(&self) -> Vec<Pubkey>;
}

impl Accounts for Pubkey {
    fn pubkeys(&self) -> Vec<Pubkey> { vec![*self] }
    fn signers(&self) -> Vec<Pubkey> { vec![] }
}

impl<K> Accounts for Signer<K> where K: Accounts {
    fn pubkeys(&self) -> Vec<Pubkey> {
        self.0.pubkeys()
    }
    fn signers(&self) -> Vec<Pubkey> {
        self.0.pubkeys()
    }
}
