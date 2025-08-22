pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::state::*;

declare_id!("2RrkEbo9XkFAgqtz5wL3xDgj3x9o6SiSw6meA9CjManb");

#[program]
pub mod dicepool {
    use super::*;

    pub fn create_pool(ctx: Context<CreateDicePoolContext>) -> Result<()> {
        _create_pool(ctx)
    }
}
