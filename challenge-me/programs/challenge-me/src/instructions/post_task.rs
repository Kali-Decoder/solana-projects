use anchor_lang::prelude::*;

use crate::{error::ChallengeError, Challenge, Task, UserProfile};

pub fn _create_task(
    ctx: Context<UploadPostTask>,
    task_name: String,
    task_description: String,
    task_emoji: String,
    current_date: String,
    day:u64,
) -> Result<()> {
    let task_account = &mut ctx.accounts.task;
    let challenge_account = &mut ctx.accounts.challenge;


    if challenge_account.completed {
        return Err(ChallengeError::AlreadyCompleted.into());
    }

    let expected_day = challenge_account.current_day + 1;

    if expected_day != (challenge_account.posts.len() as u32 + 1) {
        challenge_account.current_day = 0;
        challenge_account.completed = false;
        return Err(ChallengeError::ChallengeReset.into());
    }

    task_account.title = task_name;
    task_account.discription = task_description;
    task_account.emoji = task_emoji;
    task_account.current_time = current_date;
    task_account.day = day;
    challenge_account.current_day += 1;
    challenge_account.posts.push(task_account.key());

    if challenge_account.current_day >= challenge_account.total_days {
        challenge_account.completed = true;
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(day:u64)]
pub struct UploadPostTask<'info> {
    #[account(mut)]
    pub owner:Signer<'info>,
    #[account(mut, has_one = owner)]
    pub challenge: Account<'info, Challenge>,
    #[account(
        init,
        payer = owner,
        seeds=[b"post".as_ref(), challenge.key().as_ref(), day.to_le_bytes().as_ref()],
        bump,
        space = 8 + Task::INIT_SPACE
    )]
    pub task : Account<'info,Task>,
    pub system_program: Program<'info, System>,
}
