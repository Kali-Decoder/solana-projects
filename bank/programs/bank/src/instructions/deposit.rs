use anchor_lang::prelude::*;
use crate::{BankAccount};

pub fn _deposit_to_user_account(ctx: Context<DepositContext>, amount: u64) -> Result<()> {
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.owner.key(),
        &ctx.accounts.bank_account.key(),
        amount
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.bank_account.to_account_info(),
        ]
    )?;

    ctx.accounts.bank_account.balance += amount;

    Ok(())

}

#[derive(Accounts)]
pub struct DepositContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub bank_account: Account<'info, BankAccount>,
    pub system_program: Program<'info, System>, // System program
}
