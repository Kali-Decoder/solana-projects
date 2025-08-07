use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct BankAccount{
    #[max_len(40)]
    pub name :String,
    pub balance : u64,
    pub owner:Pubkey
}