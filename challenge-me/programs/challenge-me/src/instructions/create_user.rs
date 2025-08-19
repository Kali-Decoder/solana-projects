use anchor_lang::prelude::*;
use crate::{UserCreated, UserProfile};

pub fn _create_user(ctx: Context<InitializeAccount>) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.owner = ctx.accounts.user.key();
    user_profile.challenges = Vec::new();
    emit!(UserCreated{
        owner : ctx.accounts.user.key(),
        bump : ctx.bumps.user_profile
    }); 
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(
        init, 
        payer = user, 
        seeds=[b"user_profile".as_ref(),user.key().as_ref()],
        bump,
        space = 8 + 32 + 4 + (32 * 10)
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
