pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2QZRJPG72Ff3RAoePAtj1RSPcmDUAmcEzrMA37Nigu1t");

#[program]
pub mod bank {
    use super::*;

    pub fn create(ctx: Context<CreateBankAccountContext>, name: String) -> Result<()> {
        _create_bank_account(ctx, name)
    }

    pub fn deposit(ctx: Context<DepositContext>, amount: u64) -> Result<()> {
        _deposit_to_user_account(ctx, amount)
    }

    pub fn withdraw(ctx: Context<WithdrawContext>, amount: u64) -> Result<()> {
        _withdraw_from_bank(ctx, amount)
    }

}
