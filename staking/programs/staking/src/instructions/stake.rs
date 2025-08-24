use anchor_lang::{prelude::*,system_program::Transfer};
use anchor_lang::system_program;
use crate::{error::StakingError, utils::update_points, StakeAccount};

pub fn _stake(ctx:Context<StakeContext>,amount:u64) ->Result<()>{
    require!(amount > 0, StakingError::NotValidAmount);
    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    // update points before stake 

    update_points(user_account, clock.unix_timestamp)?;

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: user_account.to_account_info(),
        },
    );

    system_program::transfer(cpi_context, amount)?;

    user_account.staked_amount = user_account.staked_amount.checked_add(amount)
    .ok_or(StakingError::Overflow)?;

    msg!("Staked {} lamports. Total staked: {}, Total points: {}", 
    amount, user_account.staked_amount, user_account.reward_amount / 1_000_000);

    Ok(())
}

#[derive(Accounts)]
pub struct StakeContext<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_account", owner.key().as_ref()],
        bump,
        has_one = owner,
        constraint = user_account.is_active @ StakingError::NotActive,
    )]
    pub user_account: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,

}