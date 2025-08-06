use anchor_lang::prelude::*;

declare_id!("81BFRU1Q74vE5oKqj9Dsiut8QXYZFom185RZcRNzrJiv");

#[program]
pub mod swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
