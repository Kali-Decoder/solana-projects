use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount{
    pub user : Pubkey , 
    pub staked_amount : u64,
    pub reward_amount : u64,
    pub last_update : i64,
    pub is_active : bool,
    pub bump : u8,
}


#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub bump: u8,
    pub total_sol: u64, // track SOL stored
    pub owner : Pubkey, // owner of the vault   
}
