use anchor_lang::prelude::*;
use crate::{BankAccount, BANK_ACCOUNT_SEED};


pub fn _create_bank_account(ctx: Context<CreateBankAccountContext>,name:String) -> Result<()> {
    let bank = &mut ctx.accounts.bank_account;
    bank.name = name;
    bank.balance=0;
    bank.owner = ctx.accounts.owner.key();
    Ok(())
}


#[derive(Accounts)]
pub struct CreateBankAccountContext<'info>{
    #[account(mut)]
    pub owner : Signer<'info>,  
    #[account(
        init,
        payer = owner,
        seeds=[BANK_ACCOUNT_SEED.as_bytes(),owner.key().as_ref()],
        bump,
        space = 8 + BankAccount::INIT_SPACE
    )]
    pub bank_account : Account<'info,BankAccount>,
    pub system_program: Program<'info, System>,
}