use anchor_lang::prelude::*;

use crate::{Challenge, ChallengeType, UserProfile};

pub fn _start_challenge(
    ctx: Context<ChallengeContext>,
    challenge_id: u64,
    challenge_type: ChallengeType,
) -> Result<()> {

    let challenge_account = &mut ctx.accounts.challenge;
    let user_account = &mut ctx.accounts.user_account;

    challenge_account.challenge_id = challenge_id;
    challenge_account.challenge_type = challenge_type;
    user_account.challenges.push(challenge_account.key());
    Ok(())
}

#[derive(Accounts)]
pub struct ChallengeContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8   // discriminator
      + 8   // challenge_id
      + 32  // owner pubkey
      + 4   // current_day (u32)
      + 4   // total_days (u32)
      + 1   // completed (bool)
      + 1   // challenge_type (enum stored as u8)
      + 4 + (32 * 365),
        seeds=[b"challenge".as_ref(),owner.key().as_ref()],
        bump
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(
        mut,
        seeds=[b"user_profile",owner.key().as_ref()],
        bump,
    )]
    pub user_account : Account<'info,UserProfile>,
    pub system_program: Program<'info, System>,
}
