use anchor_lang::prelude::*;

use crate::StakeAccount;
pub fn _initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
    let stake_account = &mut ctx.accounts.user_account;
    let owner = &ctx.accounts.owner;
    stake_account.owner = owner.key();
    stake_account.staked_amount = 0;
    stake_account.reward_amount = 0;
    stake_account.last_update = Clock::get()?.unix_timestamp;
    stake_account.is_active = true;
    stake_account.bump = ctx.bumps.user_account;
    msg!("Stake account initialized for owner: {}", owner.key());
    Ok(())
}
#[derive(Accounts)]
pub struct InitializeAccount<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake_account", owner.key().as_ref()],
        bump
    )]
    pub user_account : Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
    
}