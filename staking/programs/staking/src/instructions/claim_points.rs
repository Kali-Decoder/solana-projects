use anchor_lang::{prelude::*};

use crate::{error::StakingError, utils::update_points, StakeAccount};

pub fn _claim_points(ctx:Context<ClaimPoints>) -> Result<()>{
    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    update_points(user_account, clock.unix_timestamp)?;
    let claimable_points = user_account.reward_amount / 1_000_000; 
    require!(claimable_points > 0, StakingError::NoPointsToClaim);
    msg!("User has {} claimable points", claimable_points);

    user_account.reward_amount = 0;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPoints<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_account", owner.key().as_ref()],
        bump,
        constraint = user_account.is_active && user_account.owner == owner.key() @ StakingError::NotActive,
    )]
    pub user_account: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
}