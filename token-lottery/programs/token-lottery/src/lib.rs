pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("88GRzzQVd95qs5TZEBAPQhm7wnkzxEY8GZV8VqkNFpqr");

#[program]
pub mod token_lottery {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeContext>,
        start_time: u64,
        end_time: u64,
        ticket_price: u64,
    ) -> Result<()> {
        _initialize_config(ctx, start_time, end_time, ticket_price)
    }

    pub fn initialize_lottery(
        ctx: Context<InitializeLottery>
    ) -> Result<()>{
        _initialize_lottery(ctx)
    }

    pub fn buy_ticket(
        ctx: Context<BuyTicketContext>
    ) -> Result<()>{
        _buy_ticket(ctx)
    }
}
