pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::state::*;

declare_id!("8pjXfmrh41TyBbiKmDaSAaKsgaStbeHt9Cjy8QskBT3M");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccount>) -> Result<()> {
        _initialize_account(ctx)
    }

    pub fn stake(ctx:Context<StakeContext>,amount:u64) -> Result<()> {
        _stake(ctx,amount)
    }

    pub fn unstake(ctx:Context<UnstakeContext>,amount:u64) -> Result<()> {
        _unstake(ctx,amount)
    }

    pub fn get_points(ctx: Context<GetPoints>) -> Result<()> {
        _get_points(ctx)
    }
    pub fn claim_points(ctx: Context<ClaimPoints>) -> Result<()> {
        _claim_points(ctx)
    }
}
