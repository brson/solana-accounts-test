use solana_program::pubkey::Pubkey as SolanaPubkey;



pub type Pubkey = SolanaPubkey;

pub struct Signer<K>(K) where K: Accounts;

pub trait Accounts {
}

impl Accounts for Pubkey {
}
