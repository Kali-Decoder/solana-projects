use anchor_lang::prelude::*;

use crate::{Challenge, ChallengeType, UserProfile};
const MAX_POSTS: usize = 100;
pub fn _start_challenge(
    ctx: Context<ChallengeContext>,
    challenge_id: u64,
    challenge_type: ChallengeType,
) -> Result<()> {

    let challenge_account = &mut ctx.accounts.challenge;
    let user_account = &mut ctx.accounts.user_account;
    let days =  match challenge_type{
        ChallengeType::OneMonth => 30,
        ChallengeType::OneYear => 365,
        ChallengeType::OneWeek => 7,
        ChallengeType::SeventyFiveHard => 75 ,
        ChallengeType::SixMonths => 180,
        ChallengeType::TwoMonths => 60
    };
    challenge_account.total_days = days;
    challenge_account.owner = *ctx.accounts.owner.key;
    challenge_account.current_day=0;
    challenge_account.completed = false;
    challenge_account.challenge_id = challenge_id;
    challenge_account.challenge_type = challenge_type;
    challenge_account.posts = Vec::new();
    msg!("Challenge PDA in Rust: {:?}", challenge_account.key());
    user_account.challenges.push(challenge_account.key());
    Ok(())
}

#[derive(Accounts)]
#[instruction(challenge_id : u64)]
pub struct ChallengeContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 8 + 32 + 4 + 4 + 1 + 1 + 4 + (32 * MAX_POSTS),
        seeds=[b"challenge".as_ref(),owner.key().as_ref(), &challenge_id.to_le_bytes()],
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
