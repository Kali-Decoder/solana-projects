use anchor_lang::{prelude::*};

use crate::Vault;
pub fn _initialize_vault(ctx: Context<InitVault>) -> Result<()> {
    let vault_account = &mut ctx.accounts.vault;
    vault_account.bump = ctx.bumps.vault;
    vault_account.total_sol = 0;
    vault_account.owner = ctx.accounts.owner.key();
    msg!("Vault account initialized");
    Ok(())
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [b"vault"],   // PDA seed
        bump,
        space = 8 + Vault::INIT_SPACE    
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}