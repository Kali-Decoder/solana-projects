use anchor_lang::prelude::*;

use crate::{error::StakingError, utils::calculate_points_earned, StakeAccount, LAMPORTS_PER_SOL};

pub fn _get_points(ctx:Context<GetPoints>) -> Result<()> {

    let pda_account = &ctx.accounts.user_account;
    let clock = Clock::get()?;

    let time_elapsed = clock.unix_timestamp.checked_sub(pda_account.last_update)
            .ok_or(StakingError::InvalidStamp)? as u64;
        
        let new_points = calculate_points_earned(pda_account.staked_amount, time_elapsed)?;
        let current_total_points = pda_account.reward_amount.checked_add(new_points)
            .ok_or(StakingError::Overflow)?;
        
        msg!("Current points: {}, Staked amount: {} SOL", 
             current_total_points / 1_000_000, 
             pda_account.staked_amount / LAMPORTS_PER_SOL);
        
        Ok(())

}


#[derive(Accounts)]
pub struct GetPoints<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"stake_account", user.key().as_ref()],
        bump = user_account.bump,
        has_one = user
    )]
    pub user_account: Account<'info, StakeAccount>,
}