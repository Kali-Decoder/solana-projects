use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

use crate::error::StakingError;
use crate::utils::update_points;
use crate::{StakeAccount, Vault};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"stake_account", user.key().as_ref()],
        bump,
        has_one = user,
        constraint = user_account.is_active && user_account.user == user.key() @ StakingError::NotActive,
    )]
    pub user_account: Account<'info, StakeAccount>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn _unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::NotValidAmount);

    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;
    update_points(user_account, clock.unix_timestamp)?;

    require!(
        user_account.staked_amount >= amount,
        StakingError::InsufficientFunds
    );

    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;
    ctx.accounts.vault.total_sol -= amount;
    user_account.staked_amount = user_account
        .staked_amount
        .checked_sub(amount)
        .ok_or(StakingError::Underflow)?;

    msg!(
        "Unstaked {} lamports. Remaining staked: {}",
        amount,
        user_account.staked_amount
    );

    Ok(())
}
