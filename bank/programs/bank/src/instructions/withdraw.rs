use anchor_lang::prelude::*;
use crate::{error::BankError,BankAccount};


pub fn _withdraw_from_bank(ctx:Context<WithdrawContext>,amount:u64) -> Result<()>{
    let bank = &mut ctx.accounts.bank_account;
    let user = &mut ctx.accounts.owner;

    if bank.owner != user.key() {
        return Err(BankError::OwnerMismatch.into()); 
    }
    
    let rent = Rent::get()?.minimum_balance(bank.to_account_info().data_len());

    // Check if there are sufficient funds in the bank account
    if **bank.to_account_info().lamports.borrow() - rent < amount {
        return Err(BankError::InsufficientFunds.into()); // Return an error if there are insufficient funds
    }

    **bank.to_account_info().try_borrow_mut_lamports()? -= amount;
    **user.to_account_info().try_borrow_mut_lamports()? += amount;

    bank.balance -= amount;
    Ok(())

}


#[derive(Accounts)]
pub struct WithdrawContext<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub bank_account: Account<'info, BankAccount>,
    pub system_program: Program<'info, System>, // System program
}