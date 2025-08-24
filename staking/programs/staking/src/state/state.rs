use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount{
    pub owner : Pubkey , 
    pub staked_amount : u64,
    pub reward_amount : u64,
    pub last_update : i64,
    pub is_active : bool,
    pub bump : u8,
}

