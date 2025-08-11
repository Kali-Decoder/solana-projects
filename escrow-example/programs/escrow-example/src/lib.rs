
use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;


pub use instructions::*;
pub use state::*;

declare_id!("A8kH1jSE9BkKMR7XDqh7YWqUQofEM5JhncsQ2dPFjcd6");

#[program]
pub mod escrow_example {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeExchange>,
        a_to_b_amount: u64,
        b_to_a_amount: u64,
        side_b: Pubkey,
    ) -> Result<()> {
        _initialize_exchange(ctx, a_to_b_amount, b_to_a_amount, side_b)
    }
}
