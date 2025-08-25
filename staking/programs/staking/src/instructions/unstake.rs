use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;

use crate::error::StakingError;
use crate::utils::update_points;
use crate::{StakeAccount, Vault};

pub fn _unstake(ctx: Context<UnstakeContext>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::NotValidAmount);

    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;
    let vault = &mut ctx.accounts.vault;
    require!(
        user_account.staked_amount >= amount,
        StakingError::InsufficientFunds
    );
    update_points(user_account, clock.unix_timestamp)?;

    let bump = vault.bump;
    let vault_seeds = &[b"vault".as_ref(), &[bump]];
    let signer_seeds = &[vault_seeds[..]];

    invoke_signed(
        &system_instruction::transfer(&vault.key(), &ctx.accounts.user.key(), amount),
        &[
            vault.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;

    vault.total_sol = vault
        .total_sol
        .checked_sub(amount)
        .ok_or(StakingError::Underflow)?;
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
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_account", user.key().as_ref()],
        bump,
        has_one = user,
        constraint = user_account.is_active && user_account.user==user.key() @ StakingError::NotActive,
    )]
    pub user_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}
