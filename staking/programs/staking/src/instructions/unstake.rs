use anchor_lang::system_program;
use anchor_lang::{prelude::*};

use crate::error::StakingError;
use crate::utils::update_points;
use crate::StakeAccount;

pub fn _unstake(ctx: Context<UnstakeContext>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::NotValidAmount);

    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    require!(
        user_account.staked_amount >= amount,
        StakingError::InsufficientFunds
    );
    update_points(user_account, clock.unix_timestamp)?;

    let seeds = &[
        b"stake_account",
        user_account.owner.as_ref(),
        &[user_account.bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: user_account.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        },
        signer,
    );
    system_program::transfer(cpi_context, amount)?;

    user_account.staked_amount = user_account
        .staked_amount
        .checked_sub(amount)
        .ok_or(StakingError::Underflow)?;

    msg!(
        "Unstaked {} lamports. Remaining staked: {}, Total points: {}",
        amount,
        user_account.staked_amount,
        user_account.reward_amount / 1_000_000
    );
    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_account", owner.key().as_ref()],
        bump,
        has_one = owner,
        constraint = user_account.is_active && user_account.owner==owner.key() @ StakingError::NotActive,
    )]
    pub user_account: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
}
