
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenLottery{
    pub bump:u8,
    pub winner : u64,
    pub winner_chosen:bool,
    pub start_time:u64,
    pub end_time:u64,
    pub lotter_pot_amount:u64,
    pub total_tickets : u64,
    pub ticket_price : u64,
    pub authority: Pubkey,
    pub randomness_account : Pubkey,
}